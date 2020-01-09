module.exports=
{
   '@tags':['newTest'],
"check Google"(browser){

    const searchTerms = "Node.js";
    const searchFiller = 'input[class="gLFyf gsfi"]';
browser
.url('https://google.com')
    .pause(2500)
    .setValue(searchFiller, searchTerms)
    .keys(browser.Keys.ENTER)
    .assert.containsText('.TbwUpd', 'https://nodejs.org')
    .pause(3000)
    }
}