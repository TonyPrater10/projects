module.exports = {
    '@disabled': true,
    '@tags':['NIH_Step2'],
    'RO, TTIPO, Medical, ORA Draft Assignments':function(browser){
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
           .setValue('input[id="txtCompanyPOC_1_1_21_1"]', ['Shalini'])
           .setValue('input[id="txtCompanyPhone_1_1_21_1"]', ['(888) 333-9999'])
           .setValue('input[id="txtCompanyEmail_1_1_21_1"]', ['tahilianis2@niaid.nih.gov'])
           .setValue('input[id="_1_1_10_1"]', ['Katalon'])
           .setValue('textarea[id="tcomments]', ['Send to TTIPO'])
           .click('input[id="btnButton2"]')
           .pause(10)


           
        },
    'NIH Step 2':function(browser){
        browser
            .url('https://edrms-qa.niaid.nih.gov/livelink/llisapi.dll?otdsauth=no-sso') // NoSSO URL
            .setValue('input[id="otds_username"]', ['testlivelink4'])
            .setValue('input[id="otds_password"]', ['Password20191010'])
            .click('input[id="loginbutton"]')
            .useXpath()
           .click("//a[contains(text(),'Division of Allergy, Immunology and Transplantation')]")
           .click("//a[contains(text(),'Office of Regulatory Affairs - Clinical Trial Agreements')]")
           .click("//a[contains(text(),'Administration')]")
           .click("//a[contains(text(),'DAIT CTA Dashboard')]")
           .click("//a[contains(text(),'TTIPO Review Draft Agreement')]")
           .useCss()
           .click('#btnButton2')


        },
    'NIH Step 3' (browser){
        browser
            .url('https://edrms-qa.niaid.nih.gov/livelink/llisapi.dll?otdsauth=no-sso') //NoSSOURL
            .setValue('input[id="otds_username"]', ['testlivelink3'])
            .setValue('input[id="otds_password"]', ['Password20191010'])
            .click('input[id="loginbutton"]')
            .useXpath()
           .click("//a[contains(text(),'Division of Allergy, Immunology and Transplantation')]")
           .click("//a[contains(text(),'Office of Regulatory Affairs - Clinical Trial Agreements')]")
           .click("//a[contains(text(),'Administration')]")
           .click("//a[contains(text(),'DAIT CTA Dashboard')]")
           .click("//a[contains(text(),'RO Prepare Draft Agreement')]")
           .click("//textarea[@id='tcomments']")
           .useCss()
           .setValue('textarea[id="tcomments]', ['Send to ORA Chief'])
           .click('#btnButton3')

    },
    
    'NIH Step 4':function (browser){
        browser
            .url('https://edrms-qa.niaid.nih.gov/livelink/llisapi.dll?otdsauth=no-sso') //NoSSOURL
            .url()
            .setValue('input[id="otds_username"]', ['testlivelink6'])
            .setValue('input[id="otds_password"]', ['Password20191010'])
            .click('input[id="loginbutton"]')
            .useXpath()
           .click("//a[contains(text(),'Division of Allergy, Immunology and Transplantation')]")
           .click("//a[contains(text(),'Office of Regulatory Affairs - Clinical Trial Agreements')]")
           .click("//a[contains(text(),'Administration')]")
           .click("//a[contains(text(),'DAIT CTA Dashboard')]")
           .click("//a[contains(text(),'ORA Chief Review Draft Agreement')]")
           .useCss()
           .setValue('textarea[id="tcomments]', ['Return to RO'])
           .click("#btnButton1")
    }
}