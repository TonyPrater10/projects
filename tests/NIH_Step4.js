
module.exports = {
     '@disabled': true,
    '@tags':['NIH_Step4'],
    'QA Review, RO Update, QA Approve':function(browser){

        browser
        .url('https://edrms-qa.niaid.nih.gov/livelink/llisapi.dll?otdsauth=no-sso')
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
            .pause(10000) // 1 sec = 10000 ms
            
       }

    },
    
    'Step 2':function(browser){

        browser 
            .url('https://edrms-qa.niaid.nih.gov/livelink/llisapi.dll?otdsauth=no-sso')
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
            .setValue('#tcomments', 'Return to QA')
            .click('#btnButton1')
            .pause(10000)
             // 1 second = 10000 ms


    }  ,
    
    'Step 4.3':function(browser){

        browser
        .url('https://edrms-qa.niaid.nih.gov/livelink/llisapi.dll?otdsauth=no-sso')
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
            .setvalue('#tcomments', 'Review Complete')
            .click('#btnButton1')
            .pause(10000) // 1 sec = 10000 ms
            
       }

    },

    'Step 4.4':function(browser){

        browser 
            .url('https://edrms-qa.niaid.nih.gov/livelink/llisapi.dll?otdsauth=no-sso')
            .assert.visible('#signin-label')
            .click('#signin-label')
            .setvalue('#otds_username', 'testlivelink3')
            .setvalue('#otds_password', 'Password20201010')
            .useXpath()
            .click("//a[contains(text(),'Division of Allergy, Immunology and Transplantation')]")
            .click("//a[contains(text(),'Office of Regulatory Affairs - Clinical Trial Agreements')]")
            .click("//a[contains(text(),'Administration')]")
            .click("//a[contains(text(),'DAIT CTA Dashboard')]")
            .click("//a[contains(text(),'RO Create Final Contract')]")
            .useCss()
            .setValue('#tcomments', 'Send to QA')
            .click('#btnButton1')
            .pause(10000)
             // 1 second = 10000 ms


    },

    'Step 4.5':function(browser){
        browser
            .url('https://edrms-qa.niaid.nih.gov/livelink/llisapi.dll?otdsauth=no-sso')    
            .useXpath()
            .click("//form[@id='thisform']/fieldset/table/tbody/tr[3]/td")
            .useCss()
            .setValue('input[id="otds_username"]', ['testlivelink1'])
            .setValue('input[id="otds_password"]', ['Password20191010'])
            .click('input[id="loginbutton"]')
            .useXpath()
            .click("//a[contains(text(),'Division of Allergy, Immunology and Transplantation')]")
            .click("//a[contains(text(),'Office of Regulatory Affairs - Clinical Trial Agreements')]")
            .click("//a[contains(text(),'Administration')]")
            .click("//a[contains(text(),'DAIT CTA Dashboard')]")
            .click("//a[contains(text(),'QA Approval of Negotiated CTA')]")
            .useCss()

       try {
            browser.acceptAlert()
       }

       finally {
            browser.switchWindow()
            .setvalue('#tcomments', 'Ready for Sign-off')
            .click('#btnButton1')
            .pause(300000) // 1 sec = 10000 ms
            
       }
    }
    

    
}