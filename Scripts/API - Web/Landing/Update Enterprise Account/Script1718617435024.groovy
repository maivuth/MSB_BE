import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testng.keyword.TestNGBuiltinKeywords as TestNGKW
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys as Keys
import groovy.json.JsonSlurper as JsonSlurper
/**
 * def loginResponse = WS.sendRequestAndVerify(findTestObject('API/Web/Login/Marker Login', [('baseUrl') : GlobalVariable.baseUrl
            , ('taxCode') : '5555923369', ('phone') : '8439555669', ('otp') : '123456']))

def jsonResponse = new JsonSlurper().parseText(loginResponse.getResponseText())

WebUI.comment(jsonResponse.toString())

// Extract data from the JSON response
bearerToken = jsonResponse.accessToken.toString()
 */


WebUI.comment(bearerToken)

WS.sendRequestAndVerify(findTestObject('API/Web/Landing/Update Enterprise Accounts', 
	[('baseUrl') : GlobalVariable.baseUrl, 
	 ('bearerToken') : bearerToken, 
	 ('consent') : true, 
	 ('taxCode') : '${taxCode}', 
	 ('companyName') : '${companyName}', 
	 ('contactPersonName') : '${contactPersonName}', 
	 ('contactPersonPhone') : '${contactPersonPhone}', 
	 ('contactPersonEmail') : '${contactPersonEmail}', 
	 ('supportStaffEmail') : '${supportStaffEmail}', 
	 ('openingAccountPurpose') : '${openingAccountPurpose}']))

