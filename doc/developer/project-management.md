# Project management

*Originally authored by Nikhil Benesch, with some addition by Robert Grimm.
Last updated October 1, 2019.*

This note addresses a scary subject: project management. The current
strategy is to impose the bare minimum amount of process to make sure everyone
knows what to work on next.

A problem I've had in other jobs is that the backlog is either non-durable,
existing only in engineers' minds, or durable but essentially infinite, where no
matter how many issues you close in a month, the issue list continues to grow.

Filing copious GitHub issues neatly solves the durability problem, but
introduces a new problem. Issues are filed for problems big and small, and
sometimes filed not for problems at all, but for open-ended discussion.
As soon as the issue list exceeds some small number, like 50 or 100, it becomes
nearly impossible to keep track of what's important and what's not. How many
of the open issues are scheduled for the next release? How many of those
are actual bugs, and how many are just new features? How should I figure out,
as an engineer, what to work on next?

To solve the issue management problem, we're currently experimenting withs
ZenHub. There's quite a bit of Agile/Kanban vocabulary ("boards", "sprints",
"epics", "story points", etc.) that can be a bit offputting, but hopefully we
can ignore most of that.

The basic idea is that we periodically create milestones with set goals. Every
bug we want fixed and every feature we want implemented is decided ahead of
time. We also create long-running releases, whose requirements are allowed to
shift over time.

ZenHub can then show us our progress towards each milestone and each release, as
well as indicate when release requirements shifted, e.g., because a serious bug
was discovered, or customer requirements mandated a change in scope.

NOTE(benesch): it's not yet clear whether we need both milestones and releases.
For now we've created a "Demo" milestone and a "Demo" release, both of which
represent the same moment in time.

The last piece of the puzzle is fastidiously assigning labels to issues. The
scheme is documented in the next section. The idea here is to make it possible
to ask questions like "are there any outstanding bugs in this release?" or "what
scheduled SQL features are missing from this release?" without doing a full
table scan.

## Issue labels

There are presently four classes of GitHub labels:

* **C**ategory labels, like **C-bug**, identify the type of issue. There are
  four categories: bugs, refactorings, features, and musings.

  Bugs, refactorings, and features should be quite familiar: bugs are defects in
  existing code, refactorings are inelegant or suboptimal pieces of code, and
  features dictate new code to be written. The line between a bug, a
  refactoring, and a feature is occasionally blurry. To distinguish, ask
  yourself, "would we be embarrased to document this issue as a known
  limitation?" If the answer is yes, it's a bug or refactoring. If the answer is
  no, it's a feature. To distinguish between bug and refactoring consider
  whether we believe the current code to be working as intended and the scope
  of the change. If it is working as extended and the scope is on the larger
  side, then chances are it's a refactoring. Also, we might occasionally
  choose to ship a release with an embarrassing bug documented as a known
  limitation, but this is usually a good litmus test nonetheless.

  Musings are issues that are intended to start discussion or record some
  thoughts, rather than serving directly as a work item. Issues are not
  necessarily the optimal forum for such discussions. You might also want to
  consider starting a mailing list thread or scheduling an in-person discussion
  at the next engineering huddle.

* **A**rea labels, like **A-sql**, identify the area of the codebase that a
  given issue impacts.

  As mentioned below, the area labels are not intended to be an exact promise of
  what modules are impacted. They're just meant to guide engineers looking for
  the next work item in a given region of the code, and to give management
  a rough sense of how many open bugs/feature requests a given area of the code
  has.

  As a result, area labels are not particularly specific. We'll need to split
  things up over time (e.g., the "integrations" area will almost certainly
  evolve to mention specific integrations), but getting too specific too soon
  increases the burden when filing an issue without much benefit.

* Theme labels, like **T-performance**, that identify the nature of the work
  required.

  Themes cut across categories and areas. For example, performance work might
  be a bug or a feature, depending upon how bad the current performance is, and
  can occur in any area.

  The intent is to help find projects for an engineer who knows what sort of
  work they're interested in (e.g., "I want to do some perf hacking; what's
  available?").

* Automatically generated labels, like **Epic** and **dependencies**. Our
  external tools insist upon adding these labels, and we don't get control over
  their text. Typically just ignore these.

* Standard labels, like **good early issue**. GitHub has several of these that
  come default with new repositories. They don't quite conform to our naming
  scheme, but it seems worthwhile to keep the default names so that external
  contributors can more easily find their way around when we make the
  repository public.

You can see the most up to date list of labels here:
https://github.com/MaterializeInc/materialize/labels.

When filing an issue, these are the rules to adhere to:

1. Assign exactly *one* category label. Is it a bug, a feature, or a musing?

2. If the category is not "musing," assign at least one area label, or more if
   the issue touches multiple areas.

   Don't worry about being exact with the area labels. If an issue is 90% a
   problem with the dataflow layer, but will have some small changes in the
   SQL layer and the glue layer, feel free to just assign **A-dataflow**.

3. Assign **good first issue** if the issue would make a good starter project
   for a new employee. You will be soundly thanked for this when the next
   employee starts!