module.exports = 
{
    '@tags':['shoutHaus'],
"Signup form for shoutHaus website"(browser)
    {
        const page = browser.page.shoutHaus();

        const formField1 = 'John';
        const formField2 = 'Doe';
        const formField3 = 'johndoe@john.com';
        const formField4 = 'JohnDoe';
        const formField5 = 'JohnDoe123';
        

       // const formFieldSelector1 ='input[name="firstname"]';
      //  const formFieldSelector2 ='input[name="lastname"]';
       // const formFieldSelector3 ='input[name="emailaddress"]';
       // const formFieldSelector4 ='input[name="username"]';
       // const formFieldSelector5 ='input[name="password"]';
      //  const submitButtonSelector='input[name="create"]';
        page
        .navigate()
        
        .FillIn(formField1)
        .FillIn(formField2)
        .FillIn(formField3)
        .FillIn(formField4)
        .FillIn(formField5)

        .submit()
        .saveScreenshot('tests_output/shouthaus.png')
        .end()
}

}