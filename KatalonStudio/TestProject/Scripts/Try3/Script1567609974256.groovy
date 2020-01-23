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

WebUI.navigateToUrl('https://otds16-dev.niaid.nih.gov/otdsws/login?RFA=a484109d-a8f4-4e9f-87b9-81e30ad91b15%3Ahttps%3A%2F%2Fedrms-dev%2Eniaid%2Enih%2Egov%2Flivelink%2Fllisapi%2Edll%3Ffunc%3Dotdsintegration%2Eredirect%26NextURL%3Dhttps%253A%252F%252Fedrms%252Ddev%252Eniaid%252Enih%252Egov%252Flivelink%252Fllisapi%252Edll%253Fotdsauth%253Dno%252Dsso&PostTicket=true&PostParams=true&otdsauth=no-sso&ux_version=1&PreserveFragment=true')

WebUI.setText(findTestObject('Page_OpenText Authentication Service/input_User name_otds_username'), 'tahilianis2')

WebUI.setEncryptedText(findTestObject('Page_OpenText Authentication Service/input_Password_otds_password'), 'Bh/QTNsNgjkD1i8nPLKQkA==')

WebUI.click(findTestObject('Page_OpenText Authentication Service/input_Password_loginbutton'))

WebUI.click(findTestObject('Page_Enterprise/a_Division of Allergy Immunology and Transplantation'))

WebUI.click(findTestObject('Page_Division of Allergy Immunology and Transplantation/a_Office of Regulatory Affairs - Clinical Trial Agreements'))

WebUI.click(findTestObject('Page_Office of Regulatory Affairs - Clinical Trial Agreements/a_CTA V3 Administration'))

WebUI.click(findTestObject('Page_CTA V3 Administration/a_Workflows and Forms'))

WebUI.click(findTestObject('Page_Workflows and Forms/a_DAIT CTA Agreement Form'))

WebUI.click(findTestObject('Page_DAIT - ORA - Clinical Trial Agreements/input_New AgreementAmend Agreement__1_1_2_1'))

WebUI.selectOptionByValue(findTestObject('Object Repository/Page_DAIT - ORA - Clinical Trial Agreements/ddAgreementType'), 
    'Clinical Trial Agreement', true)

WebUI.click(findTestObject('Object Repository/Page_DAIT - ORA - Clinical Trial Agreements/input_YesNo__1_1_20_1'))

WebUI.selectOptionByValue(findTestObject('Object Repository/Page_DAIT - ORA - Clinical Trial Agreements/ddCompany'), '3SBio', 
    true)

WebUI.selectOptionByValue(findTestObject('Object Repository/Page_DAIT - ORA - Clinical Trial Agreements/ddProduct'), 'AEOL 10150', 
    true)

WebUI.click(findTestObject('Object Repository/Page_DAIT - ORA - Clinical Trial Agreements/img_Regulatory Officer__1_1_5_1_User'))

WebUI.switchToWindowTitle('Select User')

WebUI.setText(findTestObject('Object Repository/Page_Select User/input_that starts with__ug_searchValue'), 'tahi')

WebUI.click(findTestObject('Object Repository/Page_Select User/input_that starts with_saveButton'))

WebUI.click(findTestObject('Object Repository/Page_Select User/a_Select'))

WebUI.click(findTestObject('Object Repository/Page_DAIT - ORA - Clinical Trial Agreements/input_Comments_btnButton1'))

WebUI.click(findTestObject('Object Repository/Page_Workflow Step DAIT CTA Agreement Workflow Map - Shalini Tahiliani - 09052019 - WorkID/input_Your workflow has been successfully initiated_processButton'))

