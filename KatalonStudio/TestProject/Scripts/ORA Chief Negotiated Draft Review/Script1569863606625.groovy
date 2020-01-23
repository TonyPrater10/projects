import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys as Keys

//Incorporated in RO Negotiated script
not_run: WebUI.openBrowser('')

not_run: WebUI.navigateToUrl('https://otds16-dev.niaid.nih.gov/otdsws/login?RFA=a484109d-a8f4-4e9f-87b9-81e30ad91b15%3Ahttps%3A%2F%2Fedrms-dev%2Eniaid%2Enih%2Egov%2Flivelink%2Fllisapi%2Edll%3Ffunc%3Dotdsintegration%2Eredirect%26NextURL%3Dhttps%253A%252F%252Fedrms%252Ddev%252Eniaid%252Enih%252Egov%252Flivelink%252Fllisapi%252Edll%253Fotdsauth%253Dno%252Dsso&PostTicket=true&PostParams=true&otdsauth=no-sso&ux_version=1&PreserveFragment=true')

not_run: WebUI.setText(findTestObject('Object Repository/Page_OpenText Authentication Service/input_User name_otds_username'), 
    'testlivelink3')

not_run: WebUI.setEncryptedText(findTestObject('Object Repository/Page_OpenText Authentication Service/input_Password_otds_password'), 
    'p4y+y39Ir5OVAku3apnoyY8A6tMr8UqZ')

not_run: WebUI.click(findTestObject('Object Repository/Page_OpenText Authentication Service/input_Password_loginbutton'))

not_run: WebUI.click(findTestObject('Object Repository/Page_Enterprise/a_Division of Allergy Immunology and Transplantation'))

not_run: WebUI.click(findTestObject('Object Repository/Page_Division of Allergy Immunology and Transplantation/a_Office of Regulatory Affairs - Clinical Trial Agreements'))

not_run: WebUI.click(findTestObject('Object Repository/Page_Office of Regulatory Affairs - Clinical Trial Agreements/a_CTA V3 Administration'))

not_run: WebUI.click(findTestObject('Object Repository/Page_CTA V3 Administration/a_DAIT CTA Dashboard'))

not_run: WebUI.click(findTestObject('Object Repository/Page_DAIT - ORA - Clinical Trial Agreements/a_RO Send Draft to Company'))

not_run: WebUI.setText(findTestObject('Object Repository/Page_DAIT - ORA - Clinical Trial Agreements/textarea_Comments_tcomments'), 
    'ORA Chief Review')

not_run: WebUI.click(findTestObject('Object Repository/Page_DAIT - ORA - Clinical Trial Agreements/input_Initiated by Katalon_btnButton4'))

not_run: WebUI.delay(10)

not_run: WebUI.closeBrowser()

not_run: WebUI.openBrowser('')

not_run: WebUI.navigateToUrl('https://otds16-dev.niaid.nih.gov/otdsws/login?RFA=a484109d-a8f4-4e9f-87b9-81e30ad91b15%3Ahttps%3A%2F%2Fedrms-dev%2Eniaid%2Enih%2Egov%2Flivelink%2Fllisapi%2Edll%3Ffunc%3Dotdsintegration%2Eredirect%26NextURL%3Dhttps%253A%252F%252Fedrms%252Ddev%252Eniaid%252Enih%252Egov%252Flivelink%252Fllisapi%252Edll%253Fotdsauth%253Dno%252Dsso&PostTicket=true&PostParams=true&otdsauth=no-sso&ux_version=1&PreserveFragment=true')

not_run: WebUI.setText(findTestObject('Object Repository/Page_OpenText Authentication Service/input_User name_otds_username'), 
    'testlivelink6')

not_run: WebUI.setEncryptedText(findTestObject('Object Repository/Page_OpenText Authentication Service/input_Password_otds_password'), 
    'p4y+y39Ir5OVAku3apnoyY8A6tMr8UqZ')

not_run: WebUI.click(findTestObject('Object Repository/Page_OpenText Authentication Service/input_Password_loginbutton'))

not_run: WebUI.click(findTestObject('Object Repository/Page_Enterprise/a_Division of Allergy Immunology and Transplantation'))

not_run: WebUI.click(findTestObject('Object Repository/Page_Division of Allergy Immunology and Transplantation/a_Office of Regulatory Affairs - Clinical Trial Agreements'))

not_run: WebUI.click(findTestObject('Object Repository/Page_Office of Regulatory Affairs - Clinical Trial Agreements/a_CTA V3 Administration'))

not_run: WebUI.click(findTestObject('Object Repository/Page_CTA V3 Administration/a_DAIT CTA Dashboard'))

not_run: WebUI.click(findTestObject('Object Repository/Page_DAIT - ORA - Clinical Trial Agreements/a_ORA Chief Review Negotiated Draft Agreement'))

not_run: try {
    not_run: WebUI.acceptAlert()

    not_run: WebUI.click(findTestObject('Object Repository/Page_Workflow Step ORA Chief Review Negotiated Draft Agreement/input_Work Item_IgnoreMe'))
}
finally { 
    not_run: WebUI.switchToDefaultContent()

    not_run: WebUI.setText(findTestObject('Object Repository/Page_DAIT - ORA - Clinical Trial Agreements/textarea_Comments_tcomments'), 
        'Return to RO')

    not_run: WebUI.click(findTestObject('Object Repository/Page_DAIT - ORA - Clinical Trial Agreements/input_Initiated by Katalon_btnButton1'))

    not_run: WebUI.delay(10)

    not_run: WebUI.closeBrowser()
}

not_run: WebUI.openBrowser('')

not_run: WebUI.navigateToUrl('https://otds16-dev.niaid.nih.gov/otdsws/login?RFA=a484109d-a8f4-4e9f-87b9-81e30ad91b15%3Ahttps%3A%2F%2Fedrms-dev%2Eniaid%2Enih%2Egov%2Flivelink%2Fllisapi%2Edll%3Ffunc%3Dotdsintegration%2Eredirect%26NextURL%3Dhttps%253A%252F%252Fedrms%252Ddev%252Eniaid%252Enih%252Egov%252Flivelink%252Fllisapi%252Edll%253Fotdsauth%253Dno%252Dsso&PostTicket=true&PostParams=true&otdsauth=no-sso&ux_version=1&PreserveFragment=true')

not_run: WebUI.setText(findTestObject('Object Repository/Page_OpenText Authentication Service/input_User name_otds_username'), 
    'testlivelink3')

not_run: WebUI.setEncryptedText(findTestObject('Object Repository/Page_OpenText Authentication Service/input_Password_otds_password'), 
    'p4y+y39Ir5OVAku3apnoyY8A6tMr8UqZ')

not_run: WebUI.click(findTestObject('Object Repository/Page_OpenText Authentication Service/input_Password_loginbutton'))

not_run: WebUI.click(findTestObject('Object Repository/Page_Enterprise/a_Division of Allergy Immunology and Transplantation'))

not_run: WebUI.click(findTestObject('Object Repository/Page_Division of Allergy Immunology and Transplantation/a_Office of Regulatory Affairs - Clinical Trial Agreements'))

not_run: WebUI.click(findTestObject('Object Repository/Page_Office of Regulatory Affairs - Clinical Trial Agreements/a_CTA V3 Administration'))

not_run: WebUI.click(findTestObject('Object Repository/Page_CTA V3 Administration/a_DAIT CTA Dashboard'))

not_run: WebUI.click(findTestObject('Object Repository/Page_DAIT - ORA - Clinical Trial Agreements/a_RO Send Draft to Company'))

not_run: WebUI.setText(findTestObject('Object Repository/Page_DAIT - ORA - Clinical Trial Agreements/textarea_Comments_tcomments'), 
    'Negotiation Complete')

not_run: WebUI.click(findTestObject('Object Repository/Page_DAIT - ORA - Clinical Trial Agreements/input_Initiated by Katalon_btnButton1'))

