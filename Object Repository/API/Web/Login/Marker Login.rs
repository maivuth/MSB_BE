<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Marker Login</name>
   <tag></tag>
   <elementGuidId>b15af487-fea9-4771-b548-c98b1823d6da</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <smartLocatorEnabled>false</smartLocatorEnabled>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>false</autoUpdateContent>
   <connectionTimeout>0</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n    \&quot;taxCode\&quot;: \&quot;${taxCode}\&quot;,\n    \&quot;phone\&quot;: \&quot;${phone}\&quot;,\n    \&quot;otp\&quot;: \&quot;${otp}\&quot;\n   }&quot;,
  &quot;contentType&quot;: &quot;text/plain&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>62c52c6c-768a-489a-bf6d-b8f47b6b98ce</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>9.5.0</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <path></path>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${baseUrl}/user/maker/authenticate</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>0</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.baseUrl</defaultValue>
      <description></description>
      <id>f5ca5274-5ae7-4cb4-bf86-acb3c05cef0f</id>
      <masked>false</masked>
      <name>baseUrl</name>
   </variables>
   <variables>
      <defaultValue>'5555923668'</defaultValue>
      <description></description>
      <id>d112cf59-969f-4555-9b77-54fa481afc07</id>
      <masked>false</masked>
      <name>taxCode</name>
   </variables>
   <variables>
      <defaultValue>'8439055668'</defaultValue>
      <description></description>
      <id>7cf49ded-7ca0-4be5-ac89-66d464dae255</id>
      <masked>false</masked>
      <name>phone</name>
   </variables>
   <variables>
      <defaultValue>'111111'</defaultValue>
      <description></description>
      <id>7ac56785-61aa-400e-8388-739fce9a7c9d</id>
      <masked>false</masked>
      <name>otp</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()

try {
	WS.verifyResponseStatusCode(response, 200)
	
	assertThat(response.getStatusCode()).isEqualTo(200)
	
	String jsPass =
	&quot;&quot;&quot;
{
  &quot;\$schema&quot;: &quot;http://json-schema.org/draft-07/schema#&quot;,
  &quot;title&quot;: &quot;Generated schema for Root&quot;,
  &quot;type&quot;: &quot;object&quot;,
  &quot;properties&quot;: {
    &quot;accessToken&quot;: {
      &quot;type&quot;: &quot;string&quot;
		},
      &quot;refreshToken&quot;: {
        &quot;type&quot;: &quot;string&quot;
		}
},
  &quot;required&quot;: [
    &quot;accessToken&quot;,
    &quot;refreshToken&quot;
  ]
};
&quot;&quot;&quot;
	boolean successful = WS.validateJsonAgainstSchema(response,jsPass)
}
catch(Exception e) {
	WebUI.comment(&quot;API failed ${e.message()}&quot;)
}</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
