
module.exports = {
    '@tags':['NIH_Step4'],
    'QA Review, RO Update, QA Approve'(browser){

        browser
        .url()
        .assert.visible('input[id="otds_username"]', 'input[id="otds_password"')
        .setValue('input[id="otds_username"]', ['testlivelink1'])
        .setValue('input[id="otds_password"]', ['Password20191010'])
        .click('input[id="loginbutton"]')
        .useXpath()
       .click("//a[contains(text(),'Division of Allergy, Immunology and Transplantation')]")
       .click("//a[contains(text(),'Office of Regulatory Affairs - Clinical Trial Agreements')]")
       .click("//a[contains(text(),'Administration')]")
       .click("//a[contains(text(),'DAIT CTA Dashboard')]")
       .click("//a[contains(text(),'QA Review Negotiated Draft Agreement')]")
       .useCss()

       try {
            browser.acceptAlert()
       }

       finally {
            browser.switchWindow()
            .setvalue('#tcomments', 'Return to RO')
            .click('#btnButton1')
            .pause(10)
            
       }

    },
    
    'Step 2'(browser){

        browser 
            .url()
            .assert.visible('#signin-label')
            .click('#signin-label')
            .setvalue('#otds_username', 'testlivelink3')
            .setvalue('#otds_password', 'Password20201010')
            .useXpath()
            .click("//a[contains(text(),'Division of Allergy, Immunology and Transplantation')]")
            .click("//a[contains(text(),'Office of Regulatory Affairs - Clinical Trial Agreements')]")
            .click("//a[contains(text(),'Administration')]")
            .click("//a[contains(text(),'DAIT CTA Dashboard')]")
            .click("//a[contains(text(),'RO Make updates requested by QA')]")
            .click("//input[@id='_1_1_7_1']")
            .useCss()
            .setValue('#tcomments')

    }       
}