module.exports = {
'@tags':['NIH_Step3'],
'RO, TTIPO, Medical, ORA Negotiated Reviews'(browser){
    browser
    .url('https://edrms-qa.niaid.nih.gov/livelink/llisapi.dll?otdsauth=no-sso')
    .assert.visible('input[id="otds_username"]', 'input[id="otds_password"')
    .setValue('input[id="otds_username"]', ['testlivelink3'])
    .setValue('input[id="otds_password"]', ['Password20191010'])
    .click('input[id="loginbutton"]')
    .useXpath()
   .click("//a[contains(text(),'Division of Allergy, Immunology and Transplantation')]")
   .click("//a[contains(text(),'Office of Regulatory Affairs - Clinical Trial Agreements')]")
   .click("//a[contains(text(),'Administration')]")
   .click("//a[contains(text(),'DAIT CTA Dashboard')]")
   .click("//a[contains(text(),'RO Prepare Draft Agreement')]")
   .useCss()
   .setValue("'#tcomments', 'TTIPO Review'")
   .click('#btnButton2')
   .pause(10)
},
    'Step 1'(browser){
        browser
        .url('https://edrms-qa.niaid.nih.gov/livelink/llisapi.dll?otdsauth=no-sso')
        .assert.visible('input[id="otds_username"]', 'input[id="otds_password"')
        .setValue('input[id="otds_username"]', ['testlivelink4'])
        .setValue('input[id="otds_password"]', ['Password20191010'])
        .click('input[id="loginbutton"]')
        .useXpath()
       .click("//a[contains(text(),'Division of Allergy, Immunology and Transplantation')]")
       .click("//a[contains(text(),'Office of Regulatory Affairs - Clinical Trial Agreements')]")
       .click("//a[contains(text(),'Administration')]")
       .click("//a[contains(text(),'DAIT CTA Dashboard')]")
       .click("//a[contains(text(),'TTIPO Review Negotiated Draft Agreement')]")
       .useCss()

       try {
            browser.acceptAlert()
       }

       finally {
            browser.switchWindow()
            .setvalue('#tcomments', 'Return to RO')
            .click('#btnButton1')
            .pause(10000)
            
       }

    },

    'Step 2'(browser){

        browser
            .url('https://edrms-qa.niaid.nih.gov/livelink/llisapi.dll?otdsauth=no-sso')
            .assert.visible('input[id="otds_username"]', 'input[id="otds_password"')
            .setValue('input[id="otds_username"]', ['testlivelink3'])
            .setValue('input[id="otds_password"]', ['Password20191010'])
            .click('input[id="loginbutton"]')
            .useXpath()
           .click("//a[contains(text(),'Division of Allergy, Immunology and Transplantation')]")
           .click("//a[contains(text(),'Office of Regulatory Affairs - Clinical Trial Agreements')]")
           .click("//a[contains(text(),'Administration')]")
           .click("//a[contains(text(),'DAIT CTA Dashboard')]")
           .click("//a[contains(text(),'RO Send Draft to Company')]")
           .useCss()
           .setValue("'#tcomments', 'Medical Officer Review'")
           .click('#btnButton3')
           .pause(10000)
        
    },

    'Step 3 num 57'(browser){
        browser
            .url('https://edrms-qa.niaid.nih.gov/livelink/llisapi.dll?otdsauth=no-sso')
            .assert.visible('input[id="otds_username"]', 'input[id="otds_password"')
            .setValue('input[id="otds_username"]', ['testlivelink5'])
            .setValue('input[id="otds_password"]', ['Password20191010'])
            .click('input[id="loginbutton"]')
            .useXpath()
           .click("//a[contains(text(),'Division of Allergy, Immunology and Transplantation')]")
           .click("//a[contains(text(),'Office of Regulatory Affairs - Clinical Trial Agreements')]")
           .click("//a[contains(text(),'Administration')]")
           .click("//a[contains(text(),'DAIT CTA Dashboard')]")
           .click("//a[contains(text(),'Medical Officer Review Negotiated Draft Agreement')]")
           .useCss()
           .setValue("'#tcomments', 'Return to RO'")
           .click('#btnButton1')
           .pause(10000)
    },

    
    'Step 4'(browser){
        browser
            .url('https://edrms-qa.niaid.nih.gov/livelink/llisapi.dll?otdsauth=no-sso')
            .assert.visible('input[id="otds_username"]', 'input[id="otds_password"')
            .setValue('input[id="otds_username"]', ['testlivelink3'])
            .setValue('input[id="otds_password"]', ['Password20191010'])
            .click('input[id="loginbutton"]')
            .useXpath()
           .click("//a[contains(text(),'Division of Allergy, Immunology and Transplantation')]")
           .click("//a[contains(text(),'Office of Regulatory Affairs - Clinical Trial Agreements')]")
           .click("//a[contains(text(),'Administration')]")
           .click("//a[contains(text(),'DAIT CTA Dashboard')]")
           .click("//a[contains(text(),'RO Send Draft to Company')]")
           .useCss()
           .setValue("'#tcomments', 'ORA Chief Review'")
           .click('#btnButton4')
           .pause(10000)
    },

    
    'Step 5'(browser){
        browser
            .url('https://edrms-qa.niaid.nih.gov/livelink/llisapi.dll?otdsauth=no-sso')
            .assert.visible('input[id="otds_username"]', 'input[id="otds_password"')
            .setValue('input[id="otds_username"]', ['testlivelink6'])
            .setValue('input[id="otds_password"]', ['Password20191010'])
            .click('input[id="loginbutton"]')
            .useXpath()
           .click("//a[contains(text(),'Division of Allergy, Immunology and Transplantation')]")
           .click("//a[contains(text(),'Office of Regulatory Affairs - Clinical Trial Agreements')]")
           .click("//a[contains(text(),'Administration')]")
           .click("//a[contains(text(),'DAIT CTA Dashboard')]")
           .click("//a[contains(text(),'ORA Chief Review Negotiated Draft Agreement')]")
           .useCss()
           .setValue("'#tcomments', 'Return to RO'")
           .click('#btnButton1')
           .pause(10000)
    },

    'Step 6'(browser){
        browser
            .url('https://edrms-qa.niaid.nih.gov/livelink/llisapi.dll?otdsauth=no-sso')
            .assert.visible('input[id="otds_username"]', 'input[id="otds_password"')
            .setValue('input[id="otds_username"]', ['testlivelink3'])
            .setValue('input[id="otds_password"]', ['Password20191010'])
            .click('input[id="loginbutton"]')
            .useXpath()
           .click("//a[contains(text(),'Division of Allergy, Immunology and Transplantation')]")
           .click("//a[contains(text(),'Office of Regulatory Affairs - Clinical Trial Agreements')]")
           .click("//a[contains(text(),'Administration')]")
           .click("//a[contains(text(),'DAIT CTA Dashboard')]")
           .click("//a[contains(text(),'RO Send Draft to Company')]")
           .useCss()
           .setValue("'#tcomments', 'Negotiation Complete'")
           .click('#btnButton1')
           .pause(10000)
    }

    

    
        

}