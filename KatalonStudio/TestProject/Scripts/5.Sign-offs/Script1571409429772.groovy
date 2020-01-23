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

not_run: WebUI.openBrowser('')

not_run: WebUI.navigateToUrl(GlobalVariable.NoSSOURL)

not_run: WebUI.setText(findTestObject('Object Repository/Page_OpenText Authentication Service/input_User name_otds_username'), 
    'testlivelink3')

not_run: WebUI.setMaskedText(findTestObject('Object Repository/Page_OpenText Authentication Service/input_Password_otds_password'), 
    GlobalVariable.Password)

not_run: WebUI.click(findTestObject('Object Repository/Page_OpenText Authentication Service/input_Password_loginbutton'))

not_run: WebUI.click(findTestObject('Object Repository/Page_Enterprise/a_Division of Allergy Immunology and Transplantation'))

not_run: WebUI.click(findTestObject('Object Repository/Page_Division of Allergy Immunology and Transplantation/a_Office of Regulatory Affairs - Clinical Trial Agreements'))

not_run: WebUI.click(findTestObject('Object Repository/Page_Office of Regulatory Affairs - Clinical Trial Agreements/a_CTA V3 Administration'))

not_run: WebUI.click(findTestObject('Object Repository/Page_CTA V3 Administration/a_DAIT CTA Dashboard'))

not_run: WebUI.click(findTestObject('Object Repository/Page_DAIT - ORA - Clinical Trial Agreements/a_RO Sign-Off'))

not_run: WebUI.click(findTestObject('Object Repository/Page_DAIT - ORA - Clinical Trial Agreements/input_Initiated by Katalon_btnButton1'))

not_run: WebUI.click(findTestObject('Object Repository/Page_Review and sign document(s)  DocuSign/button_Continue'))

not_run: WebUI.click(findTestObject('Object Repository/Page_Review and sign document(s)  DocuSign/button_Start'))

not_run: try {
    not_run: WebUI.click(findTestObject('Page_Review and sign document(s)  DocuSign/svg_Sign_tab-image tab-image-for-signature_1'))

    not_run: WebUI.click(findTestObject('Page_Review and sign document(s)  DocuSign/polygon_Sign_tab-image-arrow_1'))
}
catch (Exception ex) {
    not_run: WebUI.click(findTestObject('Page_Review and sign document(s)  DocuSign/polygon_Sign_tab-image-arrow_1'))
} 

not_run: WebUI.click(findTestObject('Object Repository/Page_Review and sign document(s)  DocuSign/button_Finish'))

not_run: WebUI.delay(10)

not_run: WebUI.closeBrowser()

not_run: WebUI.openBrowser('')

not_run: WebUI.navigateToUrl(GlobalVariable.NoSSOURL)

not_run: WebUI.setText(findTestObject('Object Repository/Page_OpenText Authentication Service/input_User name_otds_username'), 
    'testlivelink4')

not_run: WebUI.setMaskedText(findTestObject('Object Repository/Page_OpenText Authentication Service/input_Password_otds_password'), 
    GlobalVariable.Password)

not_run: WebUI.click(findTestObject('Object Repository/Page_OpenText Authentication Service/input_Password_loginbutton'))

not_run: WebUI.click(findTestObject('Object Repository/Page_Enterprise/a_Division of Allergy Immunology and Transplantation'))

not_run: WebUI.click(findTestObject('Object Repository/Page_Division of Allergy Immunology and Transplantation/a_Office of Regulatory Affairs - Clinical Trial Agreements'))

not_run: WebUI.click(findTestObject('Object Repository/Page_Office of Regulatory Affairs - Clinical Trial Agreements/a_CTA V3 Administration'))

not_run: WebUI.click(findTestObject('Object Repository/Page_CTA V3 Administration/a_DAIT CTA Dashboard'))

not_run: WebUI.click(findTestObject('Object Repository/Page_DAIT - ORA - Clinical Trial Agreements/a_TTIPO Sign-Off'))

not_run: WebUI.acceptAlert()

not_run: WebUI.click(findTestObject('Object Repository/Page_DAIT - ORA - Clinical Trial Agreements/input_Initiated by Katalon_btnButton1'))

not_run: WebUI.click(findTestObject('Object Repository/Page_Review and sign document(s)  DocuSign/button_Continue'))

not_run: WebUI.click(findTestObject('Object Repository/Page_Review and sign document(s)  DocuSign/span_Start'))

not_run: try {
    not_run: WebUI.click(findTestObject('Page_Review and sign document(s)  DocuSign/svg_Sign_tab-image tab-image-for-signature_2'))

    not_run: WebUI.click(findTestObject('Page_Review and sign document(s)  DocuSign/polygon_Sign_tab-image-arrow_1'))
}
catch (Exception ex) {
    not_run: WebUI.click(findTestObject('Page_Review and sign document(s)  DocuSign/svg_Sign_tab-image tab-image-for-signature_2'))
} 

not_run: WebUI.click(findTestObject('Object Repository/Page_Review and sign document(s)  DocuSign/button_Finish'))

not_run: WebUI.delay(20)

not_run: WebUI.closeBrowser()

not_run: WebUI.openBrowser('')

not_run: WebUI.navigateToUrl(GlobalVariable.NoSSOURL)

not_run: WebUI.setText(findTestObject('Object Repository/Page_OpenText Authentication Service/input_User name_otds_username'), 
    'testlivelink5')

not_run: WebUI.setMaskedText(findTestObject('Object Repository/Page_OpenText Authentication Service/input_Password_otds_password'), 
    GlobalVariable.Password)

not_run: WebUI.click(findTestObject('Object Repository/Page_OpenText Authentication Service/input_Password_loginbutton'))

not_run: WebUI.click(findTestObject('Object Repository/Page_Enterprise/a_Division of Allergy Immunology and Transplantation'))

not_run: WebUI.click(findTestObject('Object Repository/Page_Division of Allergy Immunology and Transplantation/a_Office of Regulatory Affairs - Clinical Trial Agreements'))

not_run: WebUI.click(findTestObject('Object Repository/Page_Office of Regulatory Affairs - Clinical Trial Agreements/a_CTA V3 Administration'))

not_run: WebUI.click(findTestObject('Object Repository/Page_CTA V3 Administration/a_DAIT CTA Dashboard'))

not_run: WebUI.click(findTestObject('Page_DAIT - ORA - Clinical Trial Agreements/a_Medical Officer Sign-Off'))

not_run: WebUI.click(findTestObject('Object Repository/Page_DAIT - ORA - Clinical Trial Agreements/input_Initiated by Katalon_btnButton1'))

not_run: WebUI.click(findTestObject('Object Repository/Page_Review and sign document(s)  DocuSign/button_Continue'))

not_run: WebUI.click(findTestObject('Object Repository/Page_Review and sign document(s)  DocuSign/span_Start'))

not_run: try {
    WebUI.click(findTestObject('Page_Review and sign document(s)  DocuSign/svg_Sign_tab-image tab-image-for-signature_1'))

    WebUI.click(findTestObject('Page_Review and sign document(s)  DocuSign/polygon_Sign_tab-image-arrow_1'))
}
catch (Exception e) {
    WebUI.click(findTestObject('Page_Review and sign document(s)  DocuSign/svg_Sign_tab-image tab-image-for-signature_1'))
} 

not_run: WebUI.click(findTestObject('Object Repository/Page_Review and sign document(s)  DocuSign/button_Finish'))

not_run: WebUI.delay(20)

not_run: WebUI.closeBrowser()

not_run: WebUI.openBrowser('')

not_run: WebUI.navigateToUrl(GlobalVariable.NoSSOURL)

not_run: WebUI.setText(findTestObject('Object Repository/Page_OpenText Authentication Service/input_User name_otds_username'), 
    'testlivelink6')

not_run: WebUI.setMaskedText(findTestObject('Object Repository/Page_OpenText Authentication Service/input_Password_otds_password'), 
    GlobalVariable.Password)

not_run: WebUI.click(findTestObject('Object Repository/Page_OpenText Authentication Service/input_Password_loginbutton'))

not_run: WebUI.click(findTestObject('Object Repository/Page_Enterprise/a_Division of Allergy Immunology and Transplantation'))

not_run: WebUI.click(findTestObject('Object Repository/Page_Division of Allergy Immunology and Transplantation/a_Office of Regulatory Affairs - Clinical Trial Agreements'))

not_run: WebUI.click(findTestObject('Object Repository/Page_Office of Regulatory Affairs - Clinical Trial Agreements/a_CTA V3 Administration'))

not_run: WebUI.click(findTestObject('Object Repository/Page_CTA V3 Administration/a_DAIT CTA Dashboard'))

not_run: WebUI.click(findTestObject('Object Repository/Page_DAIT - ORA - Clinical Trial Agreements/a_ORA Chief Sign-Off'))

not_run: WebUI.click(findTestObject('Object Repository/Page_DAIT - ORA - Clinical Trial Agreements/input_Initiated by Katalon_btnButton1'))

not_run: WebUI.click(findTestObject('Object Repository/Page_Review and sign document(s)  DocuSign/button_Continue'))

not_run: WebUI.click(findTestObject('Object Repository/Page_Review and sign document(s)  DocuSign/button_Start'))

not_run: try {
    not_run: WebUI.click(findTestObject('Page_Review and sign document(s)  DocuSign/svg_Sign_tab-image tab-image-for-signature_1'))

    not_run: WebUI.click(findTestObject('Page_Review and sign document(s)  DocuSign/polygon_Sign_tab-image-arrow_1'))
}
catch (Exception e) {
    not_run: WebUI.click(findTestObject('Page_Review and sign document(s)  DocuSign/svg_Sign_tab-image tab-image-for-signature_1'))
} 

not_run: WebUI.click(findTestObject('Object Repository/Page_Review and sign document(s)  DocuSign/button_Finish'))

not_run: WebUI.delay(20)

not_run: WebUI.closeBrowser()

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

WebUI.click(findTestObject('Object Repository/Page_DAIT - ORA - Clinical Trial Agreements/a_QA Sign-Off of Routing Approval Form'))

WebUI.acceptAlert()

WebUI.click(findTestObject('Object Repository/Page_DAIT - ORA - Clinical Trial Agreements/input_Initiated by Katalon_btnButton1'))

WebUI.click(findTestObject('Object Repository/Page_Review and sign document(s)  DocuSign/button_Continue'))

WebUI.click(findTestObject('Object Repository/Page_Review and sign document(s)  DocuSign/span_Start'))

try {
    WebUI.click(findTestObject('Page_Review and sign document(s)  DocuSign/svg_Sign_tab-image tab-image-for-signature'))

    WebUI.click(findTestObject('Page_Review and sign document(s)  DocuSign/polygon_Sign_tab-image-arrow_1'))
}
catch (Exception e) {
    WebUI.click(findTestObject('Page_Review and sign document(s)  DocuSign/svg_Sign_tab-image tab-image-for-signature'))
} 

WebUI.click(findTestObject('Object Repository/Page_Review and sign document(s)  DocuSign/button_Finish'))

WebUI.delay(10)

WebUI.closeBrowser()

