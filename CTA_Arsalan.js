module.exports = {
    '@tags':['CTA_Arsalan'],
    'CTA Arsalan': function (browser){
       
        const text = 'Arsalan Automation Test';


       const input ='input[name="tcomments"]';
       
        browser

        
            .url('https://otds16-dev.niaid.nih.gov/otdsws/login?RFA=a484109d%2Da8f4%2D4e9f%2D87b9%2D81e30ad91b15%3Ahttps%3A%2F%2Fedrms%2Ddev%2Eniaid%2Enih%2Egov%2Flivelink%2Fllisapi%2Edll%3Ffunc%3Dotdsintegration%2Eredirect%26NextURL%3Dhttps%253A%252F%252Fedrms%252Ddev%252Eniaid%252Enih%252Egov%252Flivelink%252Fllisapi%252Edll%253Fotdsauth%253Dno%252Dsso&PostTicket=true&PostParams=true&otdsauth=no-sso&ux_version=1&PreserveFragment=true')
            .setValue('#otds_username', 'testlivelink1') // set username
            .setValue('#otds_password', 'Password20200101') // set password
            .click('#loginbutton') // login
            .useXpath()
            .click("//a[contains(text(),'Division of Allergy, Immunology and Transplantation')]")
           .click("//a[contains(text(),'Office of Regulatory Affairs - Clinical Trial Agreements')]")
           .click("//a[contains(text(),'Administration')]")
           .click("//a[contains(text(),'DAIT CTA Dashboard')]")
           .url("https://edrms-dev.niaid.nih.gov/livelink/llisapi.dll/displayform/79940375/?viewid=79968935&readonly=true&sedit=false&objId=79973261&objAction=EditForm&nexturl=%2Flivelink%2Fllisapi%2Edll%3Ffunc%3Dll%26objId%3D79934937%26objAction%3DRunReport%26nexturl%3D%252Flivelink%252Fllisapi%252Edll%253Ffunc%253Dll%2526objId%253D79810217%2526objAction%253Dbrowse%2526viewType%253cD1")
           .useCss()
           .click('#_1_1_26_1 option[value="Clinical Trial Agreement"]') // Agreement Type
           .click('.tdvalue input[id="_1_1_20_1"]') // Multi-Company
           .useCss()
            .click('#ddCompany_1_1_21_1 option[value="3SBio"]') // Company
            .click('#ddProduct_1_1_21_1_24_1 option[value="AB103"]') // Product Name
            .setValue('#_1_1_10_1', '1234567') // Protocol Number
            .setValue('#_1_1_9_1', 'NIH') // Network Name
            .click('select[name="dd_1_1_5_1"] option[value="3553911"]') // Regulatory Officer
            .click('select[name="dd_1_1_6_1"] option[value="16400079"]') // Medical Officer
            .assert.visible('#tcomments')
            .setValue('textarea[id="tcomments"]', text) // set comments to Arsalan Automation Test
            .saveScreenshot('tests_output/CTA.png') // evidence
            .click('#btnButton1') // login

    }
}


