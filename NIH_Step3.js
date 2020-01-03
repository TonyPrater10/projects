module.exports = {
'@tags':['NIH_Step3'],
'NIH Step 3'(browser){
    browser
    .url()
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
   .setValue("'#tcomments', 'TTIPO Review'")
   .click('#btnButton2')
   .useCss()
   .pause(10)
},
    'Step 2'(browser){
        browser
        .url()
        .assert.visible('input[id="otds_username"]', 'input[id="otds_password"')
        .setValue('input[id="otds_username"]', ['testlivelink3'])
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
            .pause(10)
            
       }

    },

    'Step 2'(browser){

        browser
            .url()
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
           .setValue("'#tcomments', 'Medical Officer Review'")
           .click('#btnButton3')
           .useCss()
           .pause(10)
        
    },

    'Step 3 num 57'(browser){
        browser
            .url()
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
           .setValue("'#tcomments', 'Return to RO'")
           .click('#btnButton1')
           .useCss()
           .pause(10)
    }
    
        

}