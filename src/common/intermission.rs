//! # Intermission code.
//!
//! The intermission is the interregnum between island raids, where players can
//! manage their fleets, buy and resell items, and do other actions that cannot
//! be done at the overworld on high seas.

// Written by:
// * Gustavo Ramos Rehermann <rehermann6046@gmail.com>
//
// (c)2025 GameCircular. Under the Cooperative Non-Violent Public License.
//
// Loot & Roam is non-violent software: you can use, redistribute,
// and/or modify it under the terms of the CNPLv6+ as found
// in the LICENSE file in the source code root directory or
// at <https://git.pixie.town/thufie/CNPL>.
//
// Loot & Roam comes with ABSOLUTELY NO WARRANTY, to the extent
// permitted by applicable law.  See the CNPL for details.

/// Buildings which can be accessible from the Intermission town map.
///
/// There are multiple 'areas' that can be accessed within an intermission.
/// This can be done diegetically (through a small 'map' with multiple
/// clickable locations), or non-diegetically (through a tab bar).
/// Non-diegetic intermission navigation will be the first kind to be
/// implemented, and diegetic navigation will be made the default further down
/// the line.
// [NOTE] Consider making non-diegetic navigation acecssible in the final release as an user preference/accessibility setting.
pub enum IntermissionBuilding {
    /// The 'top' of the intermission, when no area is selected.
    ///
    /// In a diegetic intermission, this can be represented as simply
    /// displaying the town map, without anything opened within it.
    #[default]
    Top,

    /// Items can be bought in the Shop, including ammunition, foods, and
    /// various ship parts. You can drag shop items into your ships, or
    /// drag items from your ships into the shop to resell them.
    ///
    /// Different shops have different resell factors (how much cheaper they'd
    /// pay for an item, versus selling it to someone else), and different,
    /// randomly generated inventory stocks. Some intermissions have multiple
    /// shops, so it is worthwhile to take a look around for the best deals and
    /// most exotic products!
    Shop,

    /// Cheap labor can be talked out of the Tavern. Rumors can also be found
    /// there, including potentially the impact of player actions in previous
    /// runs...
    ///
    /// Higher level labor can be hired from the (Seafarers) [Guild] instead.
    Tavern,

    /// The Seafarers' Guild has skilled, but expensive, crew. Useful for
    /// manning heavy-duty parts (like the Chain Cannon).
    ///
    /// Crew hired here has higher stats, including skill stats specialized on
    /// different types of weapons. They're also more prone to striking in
    /// your ship; guildsmen know their worth.
    ///
    /// Not all intermissions are guaranteed to have a Seafarers' Guild.
    Guild,

    /// The Drydock is the only place where you can make mechanical
    /// modifications to your ship. Naturally, parts can be installed and
    /// reinstalled, and inventory can be moved around between fleet ships
    /// more easily here.
    Drydock,

    /// New ships can be checked out at the Harbor. Bigger ships are tougher
    /// and sport more slots for installing parts on them, but require more and
    /// beefier engines to propel them effectively.
    ///
    /// Not all intermissions are guaranteed to have a Harbor.
    Harbor,

    /// Information gathered while visiting town can be used to decide which
    /// island to raid next. You need enough fuel and food to make the trip to
    /// an island (measured in days), before you can select it.
    ///
    /// Multiple islands can be assessed, but only one can be picked. Some are
    /// more well defended and patrolled, but have bigger loot. There are
    /// multiple kinds of islands to raid, from small settlements to large
    /// military bases, which are generated and described accordingly.
    ///
    /// Island options are generated the moment you step in town, and cannot be
    /// rerolled. A few islands can be seen right away, but some options
    /// (usually further away) would be
    Observatory,
}
