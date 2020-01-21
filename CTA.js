module.exports = {
    '@tags':['CTA'],
    'CTA Arsalan': function (browser){

        const agency = 'NIH'
        const comment = 'Arsalan Automation';
        const randomNum = '1234567';


        const page = browser.page.CTA_Arsalan2();
        
        page
            .navigate()
            .login()
            .gotoDAIT()
            .selectCTA('@CTA', 'CLinical Trial Agreements')
            .selectCompany('@company', '3SBio')
            .selectProduct('@product', 'AB103')
            .setRandomNum(randomNum)
            .setAgency(agency)
            .makeComment(comment)

    }
}