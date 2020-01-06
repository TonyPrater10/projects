module.exports = {
    '@tags': ['GenerateAgreement'],
    'AgreementTest'(browser){
        browser
            .url()
            .useXpath()
            .click("//a[contains(text(),'New Agreement')]")
            .useCss()
            .click('#_1_1_26_1, option[value="Clinical Trial Agreement"]')
            .click("input[id='_1_1_20_1']")
            .click('#_1_1_21_1 option[value="Amgen"]')
            .click('#ddCompany_1_1_21_1_24_1 option[value="ALXN-4100 \(TPO\)"]')
            .click('#dd_1_1_5_1 option[value="3553911"]')
            .click('#dd_1_1_5_1 option[va6ue="16400079"]')
            .setValue("'#tcomments', 'Initiated by Katalon'")
            .click('#btnButton1')
            .pause(10000)
            .close()
            .pause(200)
    
       
        },
        
        'Step 2' (browser){

            browser
                .url()
                .useXpath()
                .click("//a[contains(text(),'Initiate Agreement Generation Process')]")

                try {

                }
                finally{
                    browser.switchWindow()
                    .setValue('#tcomments', 'Send to RO')
                    .click('#btnButton1')
                    .pause(10000)
                    .close()

                }
        }
    
}