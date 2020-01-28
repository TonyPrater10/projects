module.exports = {
    '@tags':['SO'],
    'new SO':function(browser){

        const searchTerm = 'Automated Testing';
        var text = 'REST Api';

        const page = browser.page.StackOverflowPO();

        page
            .navigate()
            .login()
            .clickHotTab()
            .setValue('@searchbar', [searchTerm, browser.Keys.ENTER]) 
            .assert.containsText('@questionHyperlink', text)
            .clickTopQuestion()
            .logout()
          //  .login()



    }

}