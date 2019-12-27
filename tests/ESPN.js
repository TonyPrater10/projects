module.exports = {
    '@tags' : [ESPNBrowser], 
    'ESPNBrowser' : function (browser) {

        browser
            .url('http://espn.com')
            .click('.scores-next .controls')
            .end()



    }



}