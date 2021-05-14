<h1>Release Tracker</h1>
This is used to track Releases posted on GitHub Releases and Post it on a Telegram Channel/Group.
<h2>Setup</h2>
<ol>
<li>Export env variables or just fill the .env file
<pre>
CHAT_ID="" //ID of the chat where you wanna set the alert to
API_TOKEN="" //API Token of the bot
REPO_LIST="" //List of repos seperated by comma(,) with format &ltusername&gt/&ltreponame&gt
</pre>
</li>
<li>Install cargo using by running
<pre>
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
</pre>
</li>
<li> And finally run the bot using <code>cargo run</code></li>
</ol>
