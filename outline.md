---


---

<h1 id="basics">Basics</h1>
<h2 id="dungeons">Dungeons</h2>
<ul>
<li>Contains some boss(es).</li>
<li>I don’t know yet how the boss will be choosen.</li>
<li>Each boss yields some rewards.</li>
</ul>
<h2 id="player">Player</h2>
<ul>
<li>Has HP, Dexterity, Strength stats for now.</li>
<li>Has own inventory of items.</li>
</ul>
<ul>
<li>Maybe Magic stat later</li>
</ul>
<h2 id="weapons">Weapons</h2>
<ul>
<li>Scale off Player’s Dex and Str stats.</li>
<li>Can have critical multiplier which increases based on some combination of stats and consumables.</li>
</ul>
<ul>
<li>I’ll maybe add Magic damage and weapons later based of Magic stat</li>
</ul>
<h2 id="armor">Armor</h2>
<ul>
<li>Doesn’t scale off anything, just provides defense.</li>
<li>Rings, chestplate, boots, necklaces…</li>
</ul>
<ul>
<li>Maybe Magic defense later</li>
</ul>
<h2 id="consumables-maybe">Consumables (*maybe)</h2>
<ul>
<li>Can increase critical chance for a player.</li>
<li>Can give player HP.</li>
<li>Etc.</li>
</ul>
<h2 id="market">Market</h2>
<ul>
<li>Items are bought for already implemented points, could maybe use other bots point system.</li>
<li>Players sell their items at any price they want.</li>
<li>Could maybe be cross-channel?</li>
<li>Could have some reputation system?</li>
</ul>
<h1 id="commands">Commands</h1>
<p><code>W&gt;&gt;</code> denotes a command that is sent using only a whisper to not pollute chat with unneeded information.<br>
<code>C&gt;&gt;</code> denotes a command that is sent using only a global chat.<br>
<code>&gt;&gt;</code> denotes a command that can be sent using both methods.</p>
<h2 id="player-management">Player management</h2>
<ul>
<li><code>W&gt;&gt;info &lt;player_name&gt;</code> gives info about player (stats, currently equipped stuff, but not whole inventory, current dungeon)</li>
<li><code>C&gt;&gt;create &lt;HP&gt; &lt;STR&gt; &lt;DEX&gt;</code> creates a RPG player with those stats. Sum of allocated stats must be equal to some number X I’ll decide on later.</li>
<li><code>&gt;&gt;weapon set &lt;weapon_name&gt;</code> are weapon management commands
<ul>
<li>same for armor</li>
</ul>
</li>
<li><code>W&gt;&gt;inventory list &lt;weapons|armor|consumables|all&gt;</code>, <code>W&gt;&gt;inventory info &lt;item_name&gt;</code> etc. are inventory management commands
<ul>
<li>can give info about weapons, armor, consumables etc.</li>
</ul>
</li>
</ul>
<h2 id="market-1">Market</h2>
<ul>
<li><code>C&gt;&gt;market sell &lt;item_name&gt; &lt;price&gt;</code>, <code>W&gt;&gt;market list &lt;weapons|armor|consumables|all&gt;</code>, <code>&gt;&gt;market buy &lt;index_of_item&gt;</code> are market management commands</li>
</ul>
<h1 id="gameplay">Gameplay</h1>
<ul>
<li>Someone initiates a dungeon with X (idk how many yet) number of players (for now random boss?)
<ul>
<li><code>C&gt;&gt; dungeon start &lt;empty|players&gt;</code>
<ul>
<li>If that someone lists players, only those players can join the dungeon.</li>
</ul>
</li>
<li>Players in chat are given minute or so to either <code>C&gt;&gt;join &lt;position&gt;</code> or <code>C&gt;&gt;deny</code>.</li>
<li>Players join into some position, position is number 1…X. Multiple players can take same position.</li>
<li>Position will be explained later.</li>
<li>If player quota is reached the game starts.</li>
</ul>
</li>
<li>Turn based combat
<ul>
<li>Player dies when HP reaches 0.</li>
<li>Everyone can do <strong>one action</strong> only.</li>
<li>You cannot change your build during the dungeon.</li>
<li>After each turn players get notified of how much damage they have done and how much damage they’ve took and what is boss’ current HP.</li>
<li>Boss attacks every player.</li>
<li>Boss does the most damage to the closest player (player with smaller position), then a bit less to second one etc. That way you can have tankers, DPSers etc.</li>
<li>Actions:
<ul>
<li><code>&gt;&gt;consume &lt;item&gt;</code> - consumes item that lasts until end of next turn</li>
<li><code>&gt;&gt;attack &lt;item&gt;</code> - player attacks with his primary, RNG+other factors used for critical attacks</li>
<li><code>&gt;&gt;defend</code> - player goes into defend stance, increasing chance of reducing taken damage</li>
<li>*maybe <code>&gt;&gt;leave</code> - player could lose some kind of ‘reputation’ (in market for example), they leave the dungeon</li>
</ul>
</li>
</ul>
</li>
<li>If players win, boss drops multiple items. They can be really rare, nothing special or really bad. I thought about giving most rare items to players that did most damage. Players also get more <strong>allocation points</strong> and possibly market reputation.</li>
<li>If players lose (all players die), they lose <strong>allocation points</strong>, don’t know how it’ll be decided which ones yet. They may also lose market reputation.</li>
</ul>

