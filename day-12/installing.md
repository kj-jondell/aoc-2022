# [Advent of Code](/)

-   [\[About\]](/2022/about)
-   [\[Events\]](/2022/events)
-   <a href="https://teespring.com/stores/advent-of-code"
    target="_blank">[Shop]</a>
-   [\[Log In\]](/2022/auth/login)

#    <span class="title-event-wrap">$year=</span>[2022](/2022)<span class="title-event-wrap">;</span>

-   [\[Calendar\]](/2022)
-   [\[AoC++\]](/2022/support)
-   [\[Sponsors\]](/2022/sponsors)
-   [\[Leaderboard\]](/2022/leaderboard)
-   [\[Stats\]](/2022/stats)

Our [sponsors](/2022/sponsors) help make Advent of Code possible:

<a href="https://whimsical.com/?ref=aoc22" target="_blank"
onclick="if(ga)ga(&#39;send&#39;,&#39;event&#39;,&#39;sponsor&#39;,&#39;sidebar&#39;,this.href);"
rel="noopener">Whimsical</a> - The visual collaboration hub for
diagramming, ideation & more.

## --- Day 12: Hill Climbing Algorithm ---

You try contacting the Elves using your <span
title="When you look up the specs for your handheld device, every field just says &quot;plot&quot;.">handheld
device</span>, but the river you're following must be too low to get a
decent signal.

You ask the device for a heightmap of the surrounding area (your puzzle
input). The heightmap shows the local area from above broken into a
grid; the elevation of each square of the grid is given by a single
lowercase letter, where `a` is the lowest elevation, `b` is the
next-lowest, and so on up to the highest elevation, `z`.

Also included on the heightmap are marks for your current position (`S`)
and the location that should get the best signal (`E`). Your current
position (`S`) has elevation `a`, and the location that should get the
best signal (`E`) has elevation `z`.

You'd like to reach `E`, but to save energy, you should do it in *as few
steps as possible*. During each step, you can move exactly one square
up, down, left, or right. To avoid needing to get out your climbing
gear, the elevation of the destination square can be *at most one
higher* than the elevation of your current square; that is, if your
current elevation is `m`, you could step to elevation `n`, but not to
elevation `o`. (This also means that the elevation of the destination
square can be much lower than the elevation of your current square.)

For example:

    Sabqponm
    abcryxxl
    accszExk
    acctuvwj
    abdefghi

Here, you start in the top-left corner; your goal is near the middle.
You could start by moving down or right, but eventually you'll need to
head toward the `e` at the bottom. From there, you can spiral around to
the goal:

    v..v<<<<
    >v.vv<<^
    .>vv>E^^
    ..v>>>^^
    ..>>>>>^

In the above diagram, the symbols indicate whether the path exits each
square moving up (`^`), down (`v`), left (`<`), or right (`>`). The
location that should get the best signal is still `E`, and `.` marks
unvisited squares.

This path reaches the goal in `31` steps, the fewest possible.

*What is the fewest steps required to move from your current position to
the location that should get the best signal?*

To play, please identify yourself via one of these services:

[\[GitHub\]](/auth/github) [\[Google\]](/auth/google)
[\[Twitter\]](/auth/twitter) [\[Reddit\]](/auth/reddit) <span
class="quiet">- [\[How Does Auth Work?\]](/about#faq_auth)</span>
