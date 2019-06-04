// Copyright 2019 Materialize, Inc. All rights reserved.
//
// This file is part of Materialize. Materialize may not be used or
// distributed without the express permission of Materialize, Inc.

//! A trivial single-node command queue that doesn't store state at all.

use futures::Stream;

use crate::clock::Clock;
use crate::glue::*;
use crate::sql;

pub fn serve(
    sql_command_receiver: UnboundedReceiver<(SqlCommand, CommandMeta)>,
    sql_response_mux: SqlResponseMux,
    dataflow_command_senders: Vec<UnboundedSender<(DataflowCommand, CommandMeta)>>,
    threads: Vec<std::thread::Thread>,
    clock: Clock,
) {
    std::thread::spawn(move || {
        let mut planner = sql::Planner::default();
        for msg in sql_command_receiver.wait() {
            let (sql_command, mut command_meta) = msg.unwrap();
            command_meta.timestamp = Some(clock.now());
            let connection_uuid = command_meta.connection_uuid;

            let (sql_response, dataflow_command) = match planner.handle_command(sql_command) {
                Ok((sql_response, dataflow_command)) => (Ok(sql_response), dataflow_command),
                Err(err) => (Err(err), None),
            };

            if let Some(dataflow_command) = dataflow_command {
                for (index, dataflow_command_sender) in dataflow_command_senders.iter().enumerate() {
                    dataflow_command_sender
                        .unbounded_send((dataflow_command.clone(), command_meta.clone()))
                        // if the dataflow server has gone down, just explode
                        .unwrap();

                    threads[index].unpark();
                }
            }

            // the response sender is allowed disappear at any time, so the error handling here is deliberately relaxed
            if let Ok(sender) = sql_response_mux.read().unwrap().sender(&connection_uuid) {
                drop(sender.unbounded_send(sql_response));
            }
        }
    });
}
