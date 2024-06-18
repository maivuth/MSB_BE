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

WebUI.callTestCase(findTestCase('API - Web/OTP/Send OTP'), [('baseUrl') : GlobalVariable.baseUrl, ('phone') : phone], FailureHandling.STOP_ON_FAILURE)

def response = WS.sendRequestAndVerify(findTestObject('API/Web/Login/Marker Login', [('baseUrl') : baseUrl, ('taxCode') : taxCode
			, ('phone') : phone, ('otp') : otp]))

def jsonResponse = new JsonSlurper().parseText(response.getResponseText())

// Extract data from the JSON response
bearerToken = jsonResponse.accessToken.toString()

WebUI.comment(bearerToken.toString())

WS.sendRequestAndVerify(findTestObject('API/Web/Choose Account/Create Registration Account', [('baseUrl') : baseUrl
			, ('bearerToken') : bearerToken
			, ('province') : province, ('district') : district, ('branch') : branch, ('accountNumber') : accountNumber, ('accountType') : accountType]))
