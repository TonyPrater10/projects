module.exports = {
    '@tags':['NIHStep2'],
    'RO, TTIPO, Medical, ORA Draft Assignments'(browser){
        const page = browser.page.CTA_Arsalan2.js();
        
        page
            .navigate()
            .login()
            .gotoDAIT()
            
    }
}