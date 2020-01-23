module.exports = {
    '@tags' : ['ESPN'], 
    'ESPN' : function (browser) {

        browser
            .url('http://espn.com')
           .waitForElementVisible('.scores-next')
           .click('.scores-next')
           .pause(2900)
           .assert.visible('.menu-nba')
           .click('.menu-nba')
           .pause(3000)
           .saveScreenshot('tests_output/ESPN.png')
           .click('#global-nav > ul > li.sports.menu-nba.hover > div > ul:nth-child(2) > li > div > ul:nth-child(3) > li:nth-child(6) > a')
            //.click('#global-nav .pillar.more-espn')
            .assert.titleContains('Washington Wizards Basketball - Wizards News, Scores, Stats, Rumors & More - ESPN')
            //.click('a[data-teamid="27"]')
            .end()



    }



}