module.exports = {
    '@tags':['NIH_Step6'],
    'QA Prepare, DAIT Sign, Send'(browser){
        browser
            .url()
            .setvalue('#otds_username', 'testlivelink3')
            .setvalue('#otds_password', 'Password20201010')
            .click('input[id="loginbutton"]')
            .useXpath()
            .click("//a[contains(text(),'Division of Allergy, Immunology and Transplantation')]")
            .click("//a[contains(text(),'Office of Regulatory Affairs - Clinical Trial Agreements')]")
            .click("//a[contains(text(),'Administration')]")
            .click("//a[contains(text(),'DAIT CTA Dashboard')]")
            .click("//a[contains(text(),'Prepare Send for DAIT Signatures')]")
            .click("//input[@id='_1_1_7_1']")
            .useCss()
            .setValue('#tcomments', 'Send for DAIT')
            .click('#btnButton1')
            .pause(10000)
            
    },

    'NIH Step 6.2'(browser){
        browser
        .setvalue('#otds_username', 'testlivelink7')
        .setvalue('#otds_password', 'Password20201010')
        .click('input[id="loginbutton"]')
        .useXpath()
        .click("//a[contains(text(),'Division of Allergy, Immunology and Transplantation')]")
        .click("//a[contains(text(),'Office of Regulatory Affairs - Clinical Trial Agreements')]")
        .click("//a[contains(text(),'Administration')]")
        .click("//a[contains(text(),'DAIT CTA Dashboard')]")
        .click("//a[contains(text(),'Director Sign Agreement')]")
        .useCss()
        .setValue('#tcomments', 'Return to QA')
        .click('#btnButton2')
        .pause(10000)
    },

    'NIH Step 6.3'(browser){
        browser
        .setvalue('#otds_username', 'testlivelink1')
        .setvalue('#otds_password', 'Password20201010')
        .click('input[id="loginbutton"]')
        .useXpath()
        .click("//a[contains(text(),'Division of Allergy, Immunology and Transplantation')]")
        .click("//a[contains(text(),'Office of Regulatory Affairs - Clinical Trial Agreements')]")
        .click("//a[contains(text(),'Administration')]")
        .click("//a[contains(text(),'DAIT CTA Dashboard')]")
        .click("//a[contains(text(),'QA Prepare Send for DAIT Signatures')]")
        .useCss()
         
        try{
            browser
            .accpetAlert()

        }

        finally{
            browser
            .switchWIndow()
            .setValue('#tcomments', 'Send for DAIT again')
            .pause(50000)
        }
    },

    'NIH Step 6.4'(browser){
        browser
        .url()
        .setvalue('#otds_username', 'testlivelink1')
        .setvalue('#otds_password', 'Password20201010')
        .click('input[id="loginbutton"]')
        .useXpath()
        .click("//a[contains(text(),'Division of Allergy, Immunology and Transplantation')]")
        .click("//a[contains(text(),'Office of Regulatory Affairs - Clinical Trial Agreements')]")
        .click("//a[contains(text(),'Administration')]")
        .click("//a[contains(text(),'DAIT CTA Dashboard')]")
        .click("//a[contains(text(),'Director Sign Agreement')]")
        .click("//input[@id='btnButton1']")
        .click("//button[@id='action-bar-btn-continue']")
        .click("//button[@id='navigate-btn']/span")
        .useCss()
        .click('.tab-image-arrow')
        .click('.tab-image-arrow_1')
        .click('.button #action-bar-btn-finish')
        .pause(10000)



    },

    'NIH Step 6.5'(browser){
        browser
            .url()
            .setvalue('#otds_username', 'testlivelink1')
        .setvalue('#otds_password', 'Password20201010')
        .click('input[id="loginbutton"]')
        .useXpath()
        .click("//a[contains(text(),'Division of Allergy, Immunology and Transplantation')]")
        .click("//a[contains(text(),'Office of Regulatory Affairs - Clinical Trial Agreements')]")
        .click("//a[contains(text(),'Administration')]")
        .click("//a[contains(text(),'DAIT CTA Dashboard')]")
        .click("//a[contains(text(),'Send for Company Signature')]")
        .useCss()
        try{
            browser
                .accpetAlert()
        }
        
        finally{
            browser
                .switchWindow()
                .setValue('#tcomments', 'Send For Company')
                .click('btnButton1')
                .pause(10000)
        }

    }
}