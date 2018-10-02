import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.checkpoint.CheckpointFactory as CheckpointFactory
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as MobileBuiltInKeywords
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testcase.TestCaseFactory as TestCaseFactory
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testdata.TestDataFactory as TestDataFactory
import com.kms.katalon.core.testobject.ObjectRepository as ObjectRepository
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WSBuiltInKeywords
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUiBuiltInKeywords
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import internal.GlobalVariable as GlobalVariable

WebUI.openBrowser('https://arcania-integ.astek.mu/Login')

WebUI.navigateToUrl('https://arcania-integ.astek.mu/Login')

WebUI.setText(findTestObject('Page_Arcania - Log In/input_Username'), 'b1@arc.com')

WebUI.setEncryptedText(findTestObject('Page_Arcania - Log In/input_Password'), 'nYgPIvXAGJo=')

WebUI.click(findTestObject('Page_Arcania - Log In/SignIn'))

WebUI.maximizeWindow()

WebUI.verifyTextPresent('DASHBOARD', false)

WebUI.click(findTestObject('Page_Arcania - Leads/Agent Leads/button_Create Lead'))

WebUI.click(findTestObject('Page_Arcania - Dashboard/Title_PrivateClient'))

WebUI.click(findTestObject('Page_Arcania - Lead Information/TitleRadioButton'))

WebUI.selectOptionByValue(findTestObject('Page_Arcania - Lead Information/TitleRadioButton'), '2', false)

WebUI.selectOptionByValue(findTestObject('Page_Arcania - Lead Information/TitleRadioButton'), '3', false)

WebUI.selectOptionByValue(findTestObject('Page_Arcania - Lead Information/TitleRadioButton'), '4', false)

WebUI.setText(findTestObject('Page_Arcania - Leads/PrivateClient/input_Name'), 'Test')

WebUI.setText(findTestObject('Page_Arcania - Leads/PrivateClient/input_Surname'), 'User')

WebUI.setText(findTestObject('Page_Arcania - Leads/PrivateClient/input_Email'), 'Test@arc.com')

WebUI.selectOptionByValue(findTestObject('Page_Arcania - Leads/PrivateClient/FlagSelection'), '', false)

WebUI.click(findTestObject('Page_Arcania - Leads/PrivateClient/FlagSelctionMAU'))

WebUI.setText(findTestObject('Page_Arcania - Leads/PrivateClient/input_phone'), '+230 5701 8545')

WebUI.click(findTestObject('Page_Arcania - Leads/PrivateClient/li_British'))

WebUI.click(findTestObject('null'))

WebUI.click(findTestObject('null'))

WebUI.click(findTestObject('Page_Arcania - Leads/Page_Arcania - Lead Creation/h2_New lead summary'))

WebUI.click(findTestObject('Page_Arcania - Leads/Page_Arcania - Lead Creation/button_Confirm'))

WebUI.click(findTestObject('Object Repository/Page_Arcania - LeadQualification/span_Yes'))

WebUI.setText(findTestObject('Object Repository/Page_Arcania - LeadQualification/input_LeadDTO.FormattedAssignm'), '23/05/2018')

WebUI.click(findTestObject('Object Repository/Page_Arcania - LeadQualification/span_e-icon e-arrow-sans-down'))

WebUI.click(findTestObject('Object Repository/Page_Arcania - LeadQualification/span_Manchester United'))

WebUI.click(findTestObject('Object Repository/Page_Arcania - LeadQualification/span_e-icon e-arrow-sans-down'))

WebUI.click(findTestObject('Object Repository/Page_Arcania - LeadQualification/li_Manchester United'))

WebUI.click(findTestObject('Object Repository/Page_Arcania - LeadQualification/span_OriginAddress_Country_dro'))

WebUI.setText(findTestObject('Object Repository/Page_Arcania - LeadQualification/input_OriginAddress_Country_in'), 'can')

WebUI.click(findTestObject('Object Repository/Page_Arcania - LeadQualification/li_Canada'))

WebUI.setText(findTestObject('Object Repository/Page_Arcania - LeadQualification/input_OriginAddress.City'), 'Toronto')

WebUI.setText(findTestObject('Object Repository/Page_Arcania - LeadQualification/input_DestinationAddress_Count'), 'mauri')

WebUI.click(findTestObject('Object Repository/Page_Arcania - LeadQualification/li_Mauritius'))

WebUI.setText(findTestObject('Object Repository/Page_Arcania - LeadQualification/input_DestinationAddress.City'), 'Port louis')

WebUI.sendKeys(findTestObject('Object Repository/Page_Arcania - LeadQualification/input_DestinationAddress.City'), Keys.chord(
        Keys.ENTER))

WebUI.click(findTestObject('Object Repository/Page_Arcania - LeadQualification/li_John SMITH'))

WebUI.click(findTestObject('Object Repository/Page_Arcania - LeadQualification/span_e-icon e-arrow-sans-down_1'))

WebUI.click(findTestObject('Object Repository/Page_Arcania - LeadQualification/li_Word of Mouth'))

WebUI.click(findTestObject('Object Repository/Page_Arcania - Dashboard/button_user-account-menu'))

WebUI.click(findTestObject('Page_Arcania - Log In/Logout'))

WebUI.closeBrowser()

