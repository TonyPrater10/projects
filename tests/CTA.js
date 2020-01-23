module.exports = {
    '@tags':['CTA'],
    'CTA Arsalan': function (browser){

        const agency = 'NIH';
        const randomNum = '1234567';
        const comment = 'Arsalan Automation';
        const yesNo = 'Yes';
       


        const page = browser.page.CTA_Arsalan2();
        
        page
            .navigate()
            .login()
            .gotoDAIT()
            .selectCTA('@CTA', 'CLinical Trial Agreements')
            .setYesNo(yesNo)
            .selectCompany('@company', '3SBio')
            .selectProduct('@product', 'AB103')
            .selectCompany1('@company1', '16400079')
            .selectProduct1('@product1', '3553911')
            .setRandomNum(randomNum)
            .setAgency(agency)
            .makeComment(comment)

    }
}