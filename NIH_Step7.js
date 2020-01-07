module.exports = {
    '@tags':['NIH_Step7'],
    'QA Update Metadata'(browser){
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
        .click("//a[contains(text(),'QA Update Metadata')]")
        .useCss()
    try{
        browser
            .accpetAlert()
            .click('input[name="IgnoreMe"]')
    }
    
    finally{
        browser
            .switchWindow()
            .click('#btnButton1')
            .useXpath()
            .click("//div[@id='contentarea']/div[4]/table/tbody/tr/td/div[2]/table/tbody/tr/td[2]/img")
            .click("//div[@id='ui-datepicker-div']/table/tbody/tr/td[3]/a")
            .useCss()
            .setValue('input[id="_1_1_12_1"]', 'Expires 9/20')
            .setValue('input[id="_1_1_15_1"]', 'Publish immediately')
            .setValue('textarea[id="_1_1_13_1"]', 'Enterr noted here')
            .setValue('textarea[id="tcomments"]', 'Workflow complete')
            .click('#btnButton1')
            .pause(10000)
            .end()
    }
    }

}