# Contribution Guidelines

Thank you very much for considering contributing to Loot & Roam! This is a
community project, first and foremost, and for that reason community input,
such as yours, is _essential_ for this project to keep going.

Nobody likes reading rules, but we all have to be on the same page if we want
the energy and effort spent and directed at said contribution to be fruitful.
We all want the same thing; this document merely serves to aid in achieving it!

So don't think of this as nagging, but rather some healthy reminders. :)

**- Gustavo (wallabra),** 1st lead administrator of the GameCircular cooperative


## Overview

Some important rules will be laid out first. It is a good idea to at least skim
them, so you know what to expect. Below the rules lay actual guidelines for
fruitful contribution. You can skip ahead to those if you're so inclined.


## Rules

### Interpersonal

Some of these can feel like common sense, but even if that is the case, laying
common understanding is important to resolve future disputes cleanly.

1. Do not harass, abuse, insult, or otherwise significantly harm another person
on the repository. Harm can be emotional, sexual, or reputational.

2. Do not blackmail or coerce other contributors or persons on the repository.
This **WILL** result in an immediate ban.

3. Do not write pejorative slurs or such highly pejorative statements targeting
a race, ethnicity, sexual orientation, sex, gender, socioeconomic standing,
nationality, place of birth, or age.

When and how these criteria apply is completely up to the discretion of the
maintainers. Luckily, those rules are designed so that it would take one
getting out of their own way to be harmful to even apply to them, which
simultaneously ensures well-intentioned people may ground their defense on
these solid rules.

TL;DR don't be a nocive jerk! :D

Note: If you're feeling a deja vu, note that the interpersonal rules are
equivalent to those on the organization's Code of Conduct. See it here:
https://codeberg.org/GameCircular/internal/src/branch/main/code-of-conduct.md


### Organizational

Here's some important stuff for anyone thinking about the cooperative's relation
to their community, whether they should join, etc.

Loot & Roam is being developed by GameCircular. We accept public contributions
under our own discretion of acceptance. While this is ultimately a GameCircular
project, joining the team is not necessary in order to contribute to the
development.

Please note that membership in the organization IS, however, necessary for
receiving financial dues (those allocated to splitting across team members);
it can also be revoked at any time by a _motion to excise_, which can be
voted by all members.

If you really want to see more about how the cooperative is structured (nerd!),
feel free to read through:
https://codeberg.org/GameCircular/internal/src/branch/main/superstructural.md

Also, note that joining the team doesn't necessarily mean your contributions
are more important or won't be scrutinized as much, though closeness with the
development process and the rest of the team certainly helps.


## Participation

<!-- [TODO] what are the commit message standards? -->

There are many ways to contribute to Loot & Roam. Many of these also apply to
other GameCircular projects.

### Direct participation

The most direct forms of contribution.

* **Programming:** If you're good at game development, in particular game
  programming in the Rust programing lagnuage and on the Bevy engine, feel free
  to prod around the innards of the code for areas where you can improve things!
  This also includes code review, which involves making an issue, instead of
  submitting an actual contribution yourself.

  Please note that it isn't as simple as writing code and tacking it on. There
  is necessarily a bit of a process to get it approved. Make a pull request, or
  an issue, which helps organize discussion, streamline the approval process,
  and track progress on it.

  If you want to _add_ things, please check the roadmap on [extra/roadmap.md].
  Granted, it is not a very comprehensive roadmap, but to help with that, there
  is the next item on the list...

* **Project Design:** If you think you're a good ideas person, feel free to
  join discussion on the project! Discussion channels listed on the below
  section.
  <!-- [TODO] which section? -->

  Note that this means mostly brainstorming. Your input will be considered,
  and may even get to influence development! But that is **not** a guarantee.
  If your idea doesn't make it, it isn't personal, so don't feel bad about it!
  Game design must first and foremost follow a clear direction.

* **Game Design:** You can extend the game design, creating new ship parts and
  hull makes, inventory items, even new mechanics! Note that these have to be
  drawn up in accordance with the rest of the team, and that they won't be
  merged into the full game unless

  1. They're fully implemented, including all required assets;

  2. The primary team at GameCircular approves these changes, deeming them
     to fit the game's direction and themes.

  Note that you don't need to merge it into the main game in order to extend
  it! Modding support is a long term goal, and modding is great for the project
  upstream, since it further energizes the community!

  Game design means more than just pitching ideas (that would be a _Feature
  Request_, listed in the next subsection). It means being able to flesh out
  their definitions, or how they work within the game systems, and to define
  the mechanics inside the game definiitions! (Talk to the team if you're
  confused.)

  There should be tooling available to make your own definitions, though they
  mostly can't make required assets on their own. Thankfully for that there is
  also a team!

* **Arts & Assets:** Fill in 'asset slots', helping flesh out the game
  experience; contribute new dynamic music tracks for the level themes using
  our Condemus system and music editor; and participate in the discussion on
  artistic direction, giving your two cents! Assets include modelling,
  texturing, and SFX work.

  Note that you can't just create random assets, drop them on our laps, and
  expect us to know what to do with them. Ideally you'll talk with the team.

* **Translation:** There are only so many languages that the main developers
  know, so getting this game out on other languages not only helps make it
  more accessible to speakers of said languages, but also opens up more
  opportunity for content creation with international audiences.

If legal agreements such as Common Licensing Agreements have to be struck,
those must be worked out first. Please see team contact.

  <!-- [TODO] again, where are the official contact channels? -->

### Indirect participation

There are many ways to help a community-oriented project besides the project
itself! Mainly - you guessed it - the community.

Some might think 'community' is a cute word, but there is strong evidence that
supports that a game with a strong and enthusiastic community lives longer.

* **Documentation:** Write tutorials and other documents to help other people
  reach the project and participate in it, whether as new players,
  contributors, modders, content creators; just generally community members
  alike! All are welcome. :)

  This also includes source code documentation. If you're acquainted with the
  code base but see any gaps in the in-source documentation, feel free to
  fill them in! We use Rust's documentation engine.

* **Content & Outreach:** There is no better way to excite new people to join
  than by producing elaborate showcases! Posting on your socials is a good way
  to spread by word of mouth, and better yet is multimedia - content creation.

  If you are a content creator and are reading this, you are more than invited
  to look at our project and play around with it. If you really like the game,
  maybe make a video about it!

  Note: while the main project isn't ready, it's probably best to instead share
  the prototype: https://wallabra.github.io/proto-lnr

* **Bug Reports & Feature Requests:** If you encounter an issue, or stumble
  upon a really big idea but the above ways to make it happen are not within
  your grasp, fret not! Use the issue tracker on the Codeberg page to post it
  as an issue: https://codeberg.org/GameCircular/loot-and-roam

  <!-- [TODO] what are the issue guidelines? what should I include in my issue? -->

  Please do not use the issue tracker for support requests; we have discussion
  channels specifically for those!


## Legal Stuff

This is important so that development can happen without any issues.

### Developer's Certificate of Origin 1.1

```text
By making a contribution to this project, I certify that:

 (a) The contribution was created in whole or in part by me and I
     have the right to submit it under the open source license
     indicated in the file; or

 (b) The contribution is based upon previous work that, to the best
     of my knowledge, is covered under an appropriate open source
     license and I have the right under that license to submit that
     work with modifications, whether created in whole or in part
     by me, under the same open source license (unless I am
     permitted to submit under a different license), as indicated
     in the file; or

 (c) The contribution was provided directly to me by some other
     person who certified (a), (b) or (c) and I have not modified
     it.

 (d) I understand and agree that this project and the contribution
     are public and that a record of the contribution (including all
     personal information I submit with it, including my sign-off) is
     maintained indefinitely and may be redistributed consistent with
     this project or the open source license(s) involved.
```
