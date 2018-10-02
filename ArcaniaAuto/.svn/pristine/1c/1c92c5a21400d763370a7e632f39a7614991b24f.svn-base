import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import org.openqa.selenium.WebDriver as WebDriver
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
import com.kms.katalon.core.webui.common.WebUiCommonHelper as WebUiCommonHelper
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUiBuiltInKeywords
import com.kms.katalon.core.webui.keyword.internal.WebUIKeywordMain as WebUIKeywordMain
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import internal.GlobalVariable as GlobalVariable

WebUI.openBrowser('https://arcania-integ.astek.mu/Login')

WebUI.navigateToUrl('https://arcania-integ.astek.mu/Login')

WebUI.maximizeWindow()

'Insert ${username} in the field username'
WebUI.setText(findTestObject('Page_Arcania - Log In/input_Username'), 'b1@arc.com')

'Insert ${password} in the field password'
WebUI.setText(findTestObject('Page_Arcania - Log In/input_Password'), 'b1')

'Click on the \'Sign in\' button'
WebUI.click(findTestObject('Page_Arcania - Log In/SignIn'))

'User dashboard is loaded'
WebUI.verifyElementPresent(findTestObject('Page_Arcania - Dashboard/h1_Dashboard'), 2, FailureHandling.CONTINUE_ON_FAILURE)

'Click on Lead icon found on the breadcrumb'
WebUI.click(findTestObject('Page_Arcania - Dashboard/i_icon-leads'))

'Internet Leads list is loaded'
WebUI.verifyElementPresent(findTestObject('Page_Arcania - Leads/InternetLeads/InternetLeadsList'), 0)

'Click on Agent Leads tab'
WebUI.click(findTestObject('Page_Arcania - Dashboard/Title_AgentLeads'))

WebUI.waitForPageLoad(5)

'Agent Leads list is loaded'
WebUI.verifyElementPresent(findTestObject('Page_Arcania - Leads/Agent Leads/AgentLeadsList'), 0)

'Click on \'Create Lead\' button found on right top of the web page'
WebUI.click(findTestObject('Page_Arcania - Leads/Agent Leads/button_Create Lead'), FailureHandling.STOP_ON_FAILURE)

WebUI.waitForPageLoad(2)

'The \'CREATE NEW RATE REQUEST\' page is loaded'
WebUI.verifyElementPresent(findTestObject('Page_Arcania - Leads/Page_Arcania - Lead Creation/CreateNewRateRequest'), 0)

'Insert ${AgentName} in the mandatory filed \'Agent Name'
WebUI.click(findTestObject('Page_Arcania - Leads/Agent Leads/AgentName'))

'Click on dropdown list and select Astek Mauritius from list'
WebUI.click(findTestObject('Page_Arcania - Leads/Agent Leads/AgentName_AstekMauritius'))

WebUI.verifyElementText(findTestObject('Page_Arcania - Leads/Page_Arcania - Lead Creation/AgentNameDropdown'), 'Astek Mauritius Ltd', 
    FailureHandling.CONTINUE_ON_FAILURE)

WebUI.click(findTestObject('Page_Arcania - Leads/Agent Leads/ContactPerson'))

WebUI.click(findTestObject('Page_Arcania - Leads/Agent Leads/ContactPerson_JackRobert'))

WebUI.verifyElementText(findTestObject('Page_Arcania - Leads/Page_Arcania - Lead Creation/ContactPersonDropdown'), 'Jack Robert', 
    FailureHandling.CONTINUE_ON_FAILURE)

'Checked one of the mandatory radio button for the Service Requested'
WebUI.click(findTestObject('Page_Arcania - Leads/Page_Arcania - Lead Creation/OriginServiceRadio'))

'Checked one of the mandatory radio button for the PMS Requested :'
WebUI.click(findTestObject('Page_Arcania - Leads/Page_Arcania - Lead Creation/PMS_Yes'))

'Click on \'Next Scope of Service\' option found on right bottom of the screen'
WebUI.click(findTestObject('Page_Arcania - Leads/Page_Arcania - Lead Creation/Scope of Service'))

WebUI.click(findTestObject('Page_Arcania - Leads/Page_Arcania - Lead Creation/DoorOrigin'))

WebUI.click(findTestObject('Page_Arcania - Log In/Account'))

WebUI.delay(5)

WebUI.click(findTestObject('Page_Arcania - Log In/Logout'))

