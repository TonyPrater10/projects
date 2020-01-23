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

WebUI.openBrowser('')

WebUI.navigateToUrl(GlobalVariable.NoSSOURL)

WebUI.setText(findTestObject('Object Repository/Page_OpenText Authentication Service/input_User name_otds_username'), 'testlivelink1')

WebUI.setMaskedText(findTestObject('Object Repository/Page_OpenText Authentication Service/input_Password_otds_password'), 
    GlobalVariable.Password)

WebUI.click(findTestObject('Object Repository/Page_OpenText Authentication Service/input_Password_loginbutton'))

WebUI.click(findTestObject('Object Repository/Page_Enterprise/a_Division of Allergy Immunology and Transplantation'))

WebUI.click(findTestObject('Object Repository/Page_Division of Allergy Immunology and Transplantation/a_Office of Regulatory Affairs - Clinical Trial Agreements'))

WebUI.click(findTestObject('Object Repository/Page_Office of Regulatory Affairs - Clinical Trial Agreements/a_CTA V3 Administration'))

WebUI.click(findTestObject('Object Repository/Page_CTA V3 Administration/a_DAIT CTA Dashboard'))

WebUI.click(findTestObject('Object Repository/Page_DAIT - ORA - Clinical Trial Agreements/a_QA Review Negotiated Draft Agreement'))

try {
    WebUI.acceptAlert()
}
finally { 
    WebUI.switchToDefaultContent()

    WebUI.setText(findTestObject('Object Repository/Page_DAIT - ORA - Clinical Trial Agreements/textarea_Comments_tcomments'), 
        'Return to RO')

    WebUI.click(findTestObject('Object Repository/Page_DAIT - ORA - Clinical Trial Agreements/input_Initiated by Katalon_btnButton2'))

    WebUI.delay(10)

    WebUI.closeBrowser()
}

WebUI.openBrowser('')

WebUI.navigateToUrl(GlobalVariable.NoSSOURL)

WebUI.click(findTestObject('Object Repository/Page_OpenText Authentication Service/div_User name'))

WebUI.setText(findTestObject('Object Repository/Page_OpenText Authentication Service/input_User name_otds_username'), 'testlivelink3')

WebUI.setMaskedText(findTestObject('Object Repository/Page_OpenText Authentication Service/input_Password_otds_password'), 
    GlobalVariable.Password)

WebUI.click(findTestObject('Object Repository/Page_OpenText Authentication Service/input_Password_loginbutton'))

WebUI.click(findTestObject('Object Repository/Page_Enterprise/a_Division of Allergy Immunology and Transplantation'))

WebUI.click(findTestObject('Object Repository/Page_Division of Allergy Immunology and Transplantation/a_Office of Regulatory Affairs - Clinical Trial Agreements'))

WebUI.click(findTestObject('Object Repository/Page_Office of Regulatory Affairs - Clinical Trial Agreements/a_CTA V3 Administration'))

WebUI.click(findTestObject('Object Repository/Page_CTA V3 Administration/a_DAIT CTA Dashboard'))

WebUI.click(findTestObject('Object Repository/Page_DAIT - ORA - Clinical Trial Agreements/a_RO Make updates requested by QA'))

WebUI.click(findTestObject('Object Repository/Page_DAIT - ORA - Clinical Trial Agreements/input_YesNo__1_1_7_1'))

WebUI.setText(findTestObject('Object Repository/Page_DAIT - ORA - Clinical Trial Agreements/textarea_Comments_tcomments'), 
    'Return to QA')

WebUI.click(findTestObject('Object Repository/Page_DAIT - ORA - Clinical Trial Agreements/input_Initiated by Katalon_btnButton1'))

WebUI.delay(10)

WebUI.closeBrowser()

WebUI.openBrowser('')

WebUI.navigateToUrl(GlobalVariable.NoSSOURL)

WebUI.setText(findTestObject('Object Repository/Page_OpenText Authentication Service/input_User name_otds_username'), 'testlivelink1')

WebUI.setMaskedText(findTestObject('Object Repository/Page_OpenText Authentication Service/input_Password_otds_password'), 
    GlobalVariable.Password)

WebUI.click(findTestObject('Object Repository/Page_OpenText Authentication Service/input_Password_loginbutton'))

WebUI.click(findTestObject('Object Repository/Page_Enterprise/a_Division of Allergy Immunology and Transplantation'))

WebUI.click(findTestObject('Object Repository/Page_Division of Allergy Immunology and Transplantation/a_Office of Regulatory Affairs - Clinical Trial Agreements'))

WebUI.click(findTestObject('Object Repository/Page_Office of Regulatory Affairs - Clinical Trial Agreements/a_CTA V3 Administration'))

WebUI.click(findTestObject('Object Repository/Page_CTA V3 Administration/a_DAIT CTA Dashboard'))

WebUI.click(findTestObject('Object Repository/Page_DAIT - ORA - Clinical Trial Agreements/a_QA Review Negotiated Draft Agreement'))

try {
    WebUI.acceptAlert() 
}
finally { 
    WebUI.switchToDefaultContent()

    WebUI.setText(findTestObject('Object Repository/Page_DAIT - ORA - Clinical Trial Agreements/textarea_Comments_tcomments'), 
        'Review Complete')

    WebUI.click(findTestObject('Object Repository/Page_DAIT - ORA - Clinical Trial Agreements/input_Initiated by Katalon_btnButton1'))

    WebUI.delay(10)

    WebUI.closeBrowser()
}

WebUI.openBrowser('')

WebUI.navigateToUrl(GlobalVariable.NoSSOURL)

WebUI.setText(findTestObject('Object Repository/Page_OpenText Authentication Service/input_User name_otds_username'), 'testlivelink3')

WebUI.setMaskedText(findTestObject('Object Repository/Page_OpenText Authentication Service/input_Password_otds_password'), 
    GlobalVariable.Password)

WebUI.click(findTestObject('Object Repository/Page_OpenText Authentication Service/input_Password_loginbutton'))

WebUI.click(findTestObject('Object Repository/Page_Enterprise/a_Division of Allergy Immunology and Transplantation'))

WebUI.click(findTestObject('Object Repository/Page_Division of Allergy Immunology and Transplantation/a_Office of Regulatory Affairs - Clinical Trial Agreements'))

WebUI.click(findTestObject('Object Repository/Page_Office of Regulatory Affairs - Clinical Trial Agreements/a_CTA V3 Administration'))

WebUI.click(findTestObject('Object Repository/Page_CTA V3 Administration/a_DAIT CTA Dashboard'))

WebUI.click(findTestObject('Object Repository/Page_DAIT - ORA - Clinical Trial Agreements/a_RO Create Final Contract'))

WebUI.setText(findTestObject('Object Repository/Page_DAIT - ORA - Clinical Trial Agreements/textarea_Comments_tcomments'), 
    'Send to QA')

WebUI.click(findTestObject('Object Repository/Page_DAIT - ORA - Clinical Trial Agreements/input_Initiated by Katalon_btnButton1'))

WebUI.delay(10)

WebUI.closeBrowser()

WebUI.openBrowser('')

WebUI.navigateToUrl(GlobalVariable.NoSSOURL)

WebUI.click(findTestObject('Object Repository/Page_OpenText Authentication Service/td_User name_prompts'))

WebUI.setText(findTestObject('Object Repository/Page_OpenText Authentication Service/input_User name_otds_username'), 'testlivelink1')

WebUI.setMaskedText(findTestObject('Object Repository/Page_OpenText Authentication Service/input_Password_otds_password'), 
    GlobalVariable.Password)

WebUI.click(findTestObject('Object Repository/Page_OpenText Authentication Service/input_Password_loginbutton'))

WebUI.click(findTestObject('Object Repository/Page_Enterprise/a_Division of Allergy Immunology and Transplantation'))

WebUI.click(findTestObject('Object Repository/Page_Division of Allergy Immunology and Transplantation/a_Office of Regulatory Affairs - Clinical Trial Agreements'))

WebUI.click(findTestObject('Object Repository/Page_Office of Regulatory Affairs - Clinical Trial Agreements/a_CTA V3 Administration'))

WebUI.click(findTestObject('Object Repository/Page_CTA V3 Administration/a_DAIT CTA Dashboard'))

WebUI.click(findTestObject('Object Repository/Page_DAIT - ORA - Clinical Trial Agreements/a_QA Approval of Negotiated CTA'))

try {
    WebUI.acceptAlert()

    not_run: WebUI.click(findTestObject('Object Repository/Page_Workflow Step QA Approval of Negotiated CTA/input_Work Item_IgnoreMe'))
}
finally { 
    WebUI.switchToDefaultContent()

    WebUI.setText(findTestObject('Object Repository/Page_DAIT - ORA - Clinical Trial Agreements/textarea_Comments_tcomments'), 
        'Ready for Sign-off')

    WebUI.click(findTestObject('Object Repository/Page_DAIT - ORA - Clinical Trial Agreements/input_Initiated by Katalon_btnButton1'))

    WebUI.delay(300)

    WebUI.closeBrowser()
}

