module.exports = 
{
    '@tags':['shoutHaus'],
"Signup form for shoutHaus website"(browser)
    {

        const formField1 = 'John';
        const formField2 = 'Doe';
        const formField3 = 'johndoe@john.com';
        const formField4 = 'JohnDoe';
        const formField5 = 'JohnDoe123';
        

        const formFieldSelector1 ='input[name="firstname"]';
        const formFieldSelector2 ='input[name="lastname"]';
        const formFieldSelector3 ='input[name="emailaddress"]';
        const formFieldSelector4 ='input[name="username"]';
        const formFieldSelector5 ='input[name="password"]';
        const submitButtonSelector='input[name="create"]';
        browser
        .url("https://shout.haus/registration.php")
        .setValue(formFieldSelector1, formField1)
        .setValue(formFieldSelector2, formField2)
        .setValue(formFieldSelector3, formField3)
        .setValue(formFieldSelector4, formField4)
        .setValue(formFieldSelector5, formField5)
        .click(submitButtonSelector)
        .saveScreenshot('tests_output/shouthaus.png')
}

}