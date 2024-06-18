package com.msb.utils

import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject

import com.kms.katalon.core.annotation.Keyword
import com.kms.katalon.core.checkpoint.Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling
import com.kms.katalon.core.testcase.TestCase
import com.kms.katalon.core.testdata.TestData
import com.kms.katalon.core.testng.keyword.TestNGBuiltinKeywords as TestNGKW
import com.kms.katalon.core.testobject.TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows

import internal.GlobalVariable
import com.kms.katalon.core.util.KeywordUtil
import org.apache.commons.lang.RandomStringUtils as RandStr
import java.util.concurrent.ThreadLocalRandom

public class StringHelper {
	/** * Get random number 
	 * @param min minimum value
	 * @param max maximum value
	 * @return random number
	 */
	@Keyword
	def randomNumber(String minimum, String maximum) {

		String maximumString = maximum.replaceAll("[^0-9]", "")
		String minimumString = minimum.replaceAll("[^0-9]", "")

		long minimumNumber = Long.parseLong(minimum)
		long maximumNumber = Long.parseLong(maximumString)

		def randomNo = ThreadLocalRandom.current().nextLong(minimumNumber+1, maximumNumber)
		KeywordUtil.markPassed("Random Number: "+ randomNo)
		return randomNo.toString()
	}
}
