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

WebUI.setText(findTestObject('Page_OpenText Authentication Service/input_User name_otds_username'), 'testlivelink1')

WebUI.setMaskedText(findTestObject('Object Repository/Page_OpenText Authentication Service/input_Password_otds_password'), 
    GlobalVariable.Password)

WebUI.click(findTestObject('Page_OpenText Authentication Service/input_Password_loginbutton'))

WebUI.click(findTestObject('Page_Enterprise/a_Division of Allergy Immunology and Transplantation'))

WebUI.click(findTestObject('Page_Division of Allergy Immunology and Transplantation/a_Office of Regulatory Affairs - Clinical Trial Agreements'))

WebUI.click(findTestObject('Page_Office of Regulatory Affairs - Clinical Trial Agreements/a_CTA V3 Administration'))

WebUI.click(findTestObject('Page_CTA V3 Administration/a_DAIT CTA Dashboard'))

WebUI.click(findTestObject('Object Repository/Page_DAIT - ORA - Clinical Trial Agreements/a_QA Update Metadata'))

try {
	WebUI.acceptAlert()

	WebUI.click(findTestObject('Object Repository/Page_Workflow Step QA Update Metadata/input_Work Item_IgnoreMe'))
}
finally {
	WebUI.switchToDefaultContent()

	WebUI.click(findTestObject('Page_DAIT - ORA - Clinical Trial Agreements/input_Initiated by Katalon_btnButton1'))
	
	WebUI.click(findTestObject('Object Repository/Page_DAIT - ORA - Clinical Trial Agreements/img_Date Executed_ui-datepicker-trigger'))
	
	WebUI.click(findTestObject('Object Repository/Page_DAIT - ORA - Clinical Trial Agreements/a_1'))
	
	WebUI.setText(findTestObject('Object Repository/Page_DAIT - ORA - Clinical Trial Agreements/input_Expiration Information__1_1_12_1'), 
	    'Expires 9/20')
	
	WebUI.setText(findTestObject('Object Repository/Page_DAIT - ORA - Clinical Trial Agreements/input_Publication Policy__1_1_15_1'), 
	    'Publish immediately')
	
	WebUI.setText(findTestObject('Object Repository/Page_DAIT - ORA - Clinical Trial Agreements/textarea_Notes__1_1_13_1'), 
	    'Enter notes here')
	
	WebUI.setText(findTestObject('Page_DAIT - ORA - Clinical Trial Agreements/textarea_Comments_tcomments'), 'Workflow complete')
	
	WebUI.click(findTestObject('Page_DAIT - ORA - Clinical Trial Agreements/input_Initiated by Katalon_btnButton1'))
	
	WebUI.delay(10)

}