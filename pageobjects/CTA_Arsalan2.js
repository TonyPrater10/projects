module.exports = {
    url: 'https://otds16-dev.niaid.nih.gov/otdsws/login?RFA=a484109d%2Da8f4%2D4e9f%2D87b9%2D81e30ad91b15%3Ahttps%3A%2F%2Fedrms%2Ddev%2Eniaid%2Enih%2Egov%2Flivelink%2Fllisapi%2Edll%3Ffunc%3Dotdsintegration%2Eredirect%26NextURL%3Dhttps%253A%252F%252Fedrms%252Ddev%252Eniaid%252Enih%252Egov%252Flivelink%252Fllisapi%252Edll%253Fotdsauth%253Dno%252Dsso&PostTicket=true&PostParams=true&otdsauth=no-sso&ux_version=1&PreserveFragment=true',
    elements: {
        username: '#otds_username',
        password: '#otds_password',
        login: '#loginbutton',

        Division_AIT: {
            locateStrategy: 'xpath',
            selector: "//a[contains(text(),'Division of Allergy, Immunology and Transplantation')]"
        },

        regulatoryOffice: {
            locateStrategy: 'xpath',
            selector: "//a[contains(text(),'Office of Regulatory Affairs - Clinical Trial Agreements')]"

        },

        administration: {
            locateStrategy: 'xpath',
            selector: "//a[contains(text(),'Administration')]"

        },

        dashboard: {
            locateStrategy: 'xpath',
            selector: "//a[contains(text(),'DAIT CTA Dashboard')]"

        },



        dropdown: {
            locateStrategy: 'xpath',
            selector: "//input[@id='_1_1_20_1'])[2]"
    },

        dropdown2: {
            locateStrategy: 'xpath',
            selector: ""

        },

        company: '#ddCompany_1_1_21_1',
        product: '#ddProduct_1_1_21_1_24_1',
        CTA: '#_1_1_26_1',
        randomNum:'#_1_1_10_1',
        agency: '#_1_1_9_1',
        company1:'#ddCompany_1_1_6_1',
        company2:'#ddProduct_1_1_21_1_24_1',
        commments: '#tcomments'
        

    },

    commands: [{

            login(){
                return this
                .setValue('@username', process.env.USERNAME) 
                .setValue('@password', process.env.PASSWORD); 

            },

            gotoDAIT(){
                return this
                .click('@Division_AIT')
                .click('@regulatoryOffice')
                .click('@administration')
                .click('@dashboard');
            },

            selectCTA(selector, value){
                return this
                    .click(selector)
                    .click(`option[value="value="${value}"]`);
            },

            selectCompany(selector, value){
                return this
                .click(selector)
                .click(`option[value="${value}"]`);
            },

            selectProduct(){
                    return this
                    .click(selector)
                    .click(`option[value="${value}"]`);
            },

            setRandomNum(value){
                return this
                .setValue('@randomNum', value);
            },

            setAgency(){
                return this
                .setValue('@agency', value);
            },

            makeComment(value){
                return this
                .setValue('@tcomments', value);
            }

                
                
           

    }]
}