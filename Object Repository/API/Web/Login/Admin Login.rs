<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Admin Login</name>
   <tag></tag>
   <elementGuidId>5213e4b5-0ddd-425a-a0a6-7cc82af53ac3</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <smartLocatorEnabled>false</smartLocatorEnabled>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>false</autoUpdateContent>
   <connectionTimeout>0</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n    \&quot;username\&quot;: \&quot;${username}\&quot;,\n    \&quot;password\&quot;: \&quot;${password}\&quot;\n}&quot;,
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
      <webElementGuid>c33bb5b7-b90d-48e4-92fe-45e2464c3acf</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>9.5.0</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <path></path>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>https://msb.kmsmoba.com/dev/api/user/admin/authenticate</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>0</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>'admin'</defaultValue>
      <description></description>
      <id>9f2770cd-cf37-4fdb-a129-8325934707f7</id>
      <masked>false</masked>
      <name>username</name>
   </variables>
   <variables>
      <defaultValue>'123456'</defaultValue>
      <description></description>
      <id>5c5ef622-e1cc-416f-8986-563b0691f9d0</id>
      <masked>false</masked>
      <name>password</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()

WS.verifyResponseStatusCode(response, 200)

assertThat(response.getStatusCode()).isEqualTo(200)

assertThat(response.getStatusCode()).isIn(Arrays.asList(200, 201, 202))

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
boolean successful = WS.validateJsonAgainstSchema(response,jsPass)</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
