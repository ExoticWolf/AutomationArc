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

WebUI.maximizeWindow()

'Insert a valid Login / password and click on Sign in button'
WebUI.setText(findTestObject('Page_Arcania - Log In/input_Username'), 'b1@arc.com')

WebUI.setText(findTestObject('Page_Arcania - Log In/input_Password'), 'b1')

WebUI.click(findTestObject('Page_Arcania - Log In/SignIn'))

WebUI.waitForPageLoad(10)

'The dashboard appear'
WebUI.waitForElementPresent(findTestObject('Page_Arcania - Dashboard/h1_Dashboard'), 0)

'Click on the menu lead on the left'
WebUI.click(findTestObject('Page_Arcania - Dashboard/i_icon-leads'))

'The LEADS page appear with the following TAB:\r\n\r\n-Private client\r\n\r\n-Internet leads\r\n\r\n-Agent leads\r\n\r\n-Corporate initiation'
WebUI.verifyElementPresent(findTestObject('Page_Arcania - Dashboard/Title_PrivateClient'), 0)

WebUI.verifyElementPresent(findTestObject('Page_Arcania - Dashboard/Title_InternetLeads'), 0)

WebUI.verifyElementPresent(findTestObject('Page_Arcania - Dashboard/Title_AgentLeads'), 0)

WebUI.verifyElementPresent(findTestObject('Page_Arcania - Dashboard/Title_CorporateInitiations'), 0)

WebUI.verifyTextPresent('LEADS', false)

'The Internet lead GRID appear correctly'
WebUI.click(findTestObject('Page_Arcania - Dashboard/Title_AgentLeads'))

WebUI.verifyElementPresent(findTestObject('Page_Arcania - Leads/Agent Leads/AgentLeadsList'), 0)

WebUI.click(findTestObject('Page_Arcania - Leads/InternetLeads/Status_Button'))

if (true) {
    WebUI.verifyOptionSelectedByLabel(findTestObject('Page_Arcania - Leads/InternetLeads/label_Pending follow-up'), 'Pending follow-up', 
        false, 0)

    continue
} else {
    WebUI.click(findTestObject('Page_Arcania - Leads/InternetLeads/label_Pending follow-up'))
}

WebUI.waitForPageLoad(5)

'Test case Pending due to lack of data'
WebUI.click(findTestObject(null))

WebUI.verifyElementText(findTestObject(null), '')

