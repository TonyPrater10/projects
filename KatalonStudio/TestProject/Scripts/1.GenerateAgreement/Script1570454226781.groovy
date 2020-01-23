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
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject

WebUI.openBrowser('')

WebUI.navigateToUrl('https://otds16-qa.niaid.nih.gov/otdsws/login?RFA=c6d25b0b%2D9c8a%2D423c%2Db86f%2D18b06ba691d0%3Ahttps%3A%2F%2Fedrms%2Dqa%2Eniaid%2Enih%2Egov%2Flivelink%2Fllisapi%2Edll%3Ffunc%3Dotdsintegration%2Eredirect%26NextURL%3Dhttps%253A%252F%252Fedrms%252Dqa%252Eniaid%252Enih%252Egov%252Flivelink%252Fllisapi%252Edll%253Fotdsauth%253Dno%252Dsso&PostTicket=true&PostParams=true&otdsauth=no-sso&ux_version=1&PreserveFragment=true')

WebUI.setText(findTestObject('Object Repository/CTA/Page_OpenText Authentication Service/input_User name_otds_username'), 
    'Testlivelink1')

WebUI.setEncryptedText(findTestObject('Object Repository/CTA/Page_OpenText Authentication Service/input_Password_otds_password'), 
    'p4y+y39Ir5PjMgFmS+bJDYymcoyhMUrt')

WebUI.click(findTestObject('Object Repository/CTA/Page_OpenText Authentication Service/input_Password_loginbutton'))

WebUI.click(findTestObject('Object Repository/CTA/Page_Enterprise/a_Division of Allergy Immunology and Transp_4fca1a'))

WebUI.click(findTestObject('Object Repository/CTA/Page_Division of Allergy Immunology and Tra_b475f2/td_Office of Regulatory Affairs - Clinical _28d1db'))

WebUI.click(findTestObject('Object Repository/CTA/Page_Division of Allergy Immunology and Tra_b475f2/a_Office of Regulatory Affairs - Clinical T_ec282d'))

WebUI.click(findTestObject('Object Repository/CTA/Page_Office of Regulatory Affairs - Clinica_03bb67/img'))

WebUI.click(findTestObject('Object Repository/CTA/Page_DAIT - ORA - Clinical Trial Agreements/a_New Agreement'))

WebUI.click(findTestObject('Object Repository/CTA/Page_DAIT - ORA - Clinical Trial Agreements/input_YesNo__1_1_20_1'))

WebUI.selectOptionByValue(findTestObject('Object Repository/CTA/Page_DAIT - ORA - Clinical Trial Agreements/select_- None -                            _0db343'), 
    'Clinical Trial Agreement', true)

WebUI.selectOptionByValue(findTestObject('Object Repository/CTA/Page_DAIT - ORA - Clinical Trial Agreements/select_- None -20  20 Genesystems3SBioABT H_046b5c'), 
    'Achaogen', true)

WebUI.selectOptionByValue(findTestObject('Object Repository/CTA/Page_DAIT - ORA - Clinical Trial Agreements/select_- None -3-D tissues (platform techno_7fb286'), 
    'Ajulemic Acid', true)

WebUI.setText(findTestObject('Object Repository/CTA/Page_DAIT - ORA - Clinical Trial Agreements/input_Protocol Number__1_1_10_1'), 
    '1234567')

WebUI.setText(findTestObject('Object Repository/CTA/Page_DAIT - ORA - Clinical Trial Agreements/input_Network Name__1_1_9_1'), 
    'NIH')

WebUI.selectOptionByValue(findTestObject('Object Repository/CTA/Page_DAIT - ORA - Clinical Trial Agreements/select_- None -Julia GoldsteinLing LiLyudmi_29e7d2'), 
    '3553911', true)

WebUI.selectOptionByValue(findTestObject('Object Repository/CTA/Page_DAIT - ORA - Clinical Trial Agreements/select_- None -Alkis TogiasEllen GoldmuntzJ_4097f6'), 
    '16400079', true)

WebUI.setText(findTestObject('Object Repository/CTA/Page_DAIT - ORA - Clinical Trial Agreements/textarea_Comments_tcomments'), 
    'Arsalan Automation Test')

WebUI.click(findTestObject('Object Repository/CTA/Page_DAIT - ORA - Clinical Trial Agreements/input_Comments_btnButton1'))

WebUI.closeBrowser()

