module.exports = {
    '@tags':['CTA_Arsalan'],
    'CTA Arsalan': function (browser){
        browser
            .url('https://otds16-dev.niaid.nih.gov/otdsws/login?RFA=a484109d%2Da8f4%2D4e9f%2D87b9%2D81e30ad91b15%3Ahttps%3A%2F%2Fedrms%2Ddev%2Eniaid%2Enih%2Egov%2Flivelink%2Fllisapi%2Edll%3Ffunc%3Dotdsintegration%2Eredirect%26NextURL%3Dhttps%253A%252F%252Fedrms%252Ddev%252Eniaid%252Enih%252Egov%252Flivelink%252Fllisapi%252Edll%253Fotdsauth%253Dno%252Dsso&PostTicket=true&PostParams=true&otdsauth=no-sso&ux_version=1&PreserveFragment=true')
            .setValue('otds_username', 'testlivelink1')
            .setValue('otds_password', 'Password20201010')
            .click('#loginbutton')
            .useXpath()
            .click("//a[contains(text(),'Division of Allergy, Immunology and Transplantation')]")
           .click("//a[contains(text(),'Office of Regulatory Affairs - Clinical Trial Agreements')]")
           .click("//a[contains(text(),'Administration')]")
           .click("//a[contains(text(),'DAIT CTA Dashboard')]")
           .click("//a[contains(text(),'New Agreement')]")
           .click("//select[@id='_1_1_26_1']")
           .click("(//input[@id='_1_1_20_1'])[2]")
           .useCss()
            .click('#ddCompany_1_1_21_1 option[value="3SBio"]')
            .click('#ddProduct_1_1_21_1_24_1 option[value="AB103"]')
            .setValue('#_1_1_10_1,', '1234567')
            .setValue('#_1_1_9_1,', 'NIH')
            .click('#ddCompany_1_1_6_1 option[value="16400079"]')
            .click('#ddProduct_1_1_21_1_24_1 option[value="3553911"]')
            .setvalue ('#tcomments', 'Arsalan Automation Test')
            .click('#btnButton1')

    }
}