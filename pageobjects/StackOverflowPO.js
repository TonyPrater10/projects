module.exports = {
    url: 'https://stackoverflow.com',

    elements: {
        homeButton:'-logo js-gps-track',
        loginMenu:'.login-link:nth-of-type(1)',
        username: 'input[name="email"]',
        password: 'input[name="password"]',
        loginButton: 'button[name=submit-button]',
        searchbar: 'input[type=text]',
        mainbar: '#mainbar',
        body:'body',
        topQuestion:'#question-summary-12135309',
        questionHyperlink: '.question-hyperlink',
        menuButton1: '.-ctas a:nth-child(2)',
        menuButton2: '.js-site-switcher-button',
        Users:'#nav-users',
        hotTab:'#mainbar .grid--cell a:nth-child(3)',
        logoutButton: '.s-btn__primary',
        logoutMenuButton: {
            locateStrategy: 'xpath',
            selector: "//*[contains(text(), 'log out')]"

        }


    },

    commands: {

        

        login(){
            return this
            .click('@loginMenu')
            .setValue('@username', process.env.stackUsername)
            .setValue('@password', process.env.stackPassword)
            .click('@loginButton')
        },
        clickHotTab(){
            return this
            .click('@hotTab')
        },
        clickTopQuestion(){
            return this
            //.waitforElementVisible('@mainbar')
            //.assert.containsText('@topQuestion', 'REST API')
            .click('@questionHyperlink')
        },
        
        logout(){
            return this
            .click('@menuButton2')
            .click('@logoutMenuButton')
            .click('@logoutButton')
        }
        


    }


}