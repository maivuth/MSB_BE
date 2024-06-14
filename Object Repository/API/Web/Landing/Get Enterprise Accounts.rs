<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Get Enterprise Accounts</name>
   <tag></tag>
   <elementGuidId>c769121f-6434-40ac-a107-47ce88f7ccd0</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <smartLocatorEnabled>false</smartLocatorEnabled>
   <useRalativeImagePath>false</useRalativeImagePath>
   <authorizationRequest>
      <authorizationInfo>
         <entry>
            <key>bearerToken</key>
            <value>eyJhbGciOiJIUzUxMiJ9.eyJzdWIiOiIxMzE2IiwicmVnaXN0cmF0aW9uX2lkIjoxMzE2LCJleHAiOjE3MjE4NzQ1NzIsImlhdCI6MTcxODI3NDU3MiwiYXV0aG9yaXR5IjoiTUFLRVJfUk9MRSJ9.zpHNHIQi-yygPWtuZSITUMUCkTTF0c1gptcZZhFndcPyJAQWLFSlnMNA9cI6TGlBNXIpBEn4fie7JXJ_6a4OfA</value>
         </entry>
      </authorizationInfo>
      <authorizationType>Bearer</authorizationType>
   </authorizationRequest>
   <autoUpdateContent>true</autoUpdateContent>
   <connectionTimeout>0</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;&quot;,
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
      <webElementGuid>d5a466d8-bb8e-4fe1-9601-e749163c3bed</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer eyJhbGciOiJIUzUxMiJ9.eyJzdWIiOiIxMzE2IiwicmVnaXN0cmF0aW9uX2lkIjoxMzE2LCJleHAiOjE3MjE4NzQ1NzIsImlhdCI6MTcxODI3NDU3MiwiYXV0aG9yaXR5IjoiTUFLRVJfUk9MRSJ9.zpHNHIQi-yygPWtuZSITUMUCkTTF0c1gptcZZhFndcPyJAQWLFSlnMNA9cI6TGlBNXIpBEn4fie7JXJ_6a4OfA</value>
      <webElementGuid>aa60f718-dd48-4840-a74e-b002ab93cfe1</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>9.5.0</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <path></path>
   <restRequestMethod>GET</restRequestMethod>
   <restUrl>https://msb.kmsmoba.com/dev/api/user/enterprise-accounts</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>0</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()


String jsonPass =
&quot;&quot;&quot;
{
  &quot;\$id&quot;: &quot;https://example.com/person.schema.json&quot;,
  &quot;\$schema&quot;: &quot;https://json-schema.org/draft/2020-12/schema&quot;,
  &quot;title&quot;: &quot;Person&quot;,
  &quot;type&quot;: &quot;object&quot;,
  &quot;properties&quot;: {
    &quot;firstName&quot;: {
      &quot;type&quot;: &quot;string&quot;,
      &quot;description&quot;: &quot;The person's first name.&quot;
    },
    &quot;lastName&quot;: {
      &quot;type&quot;: &quot;string&quot;,
      &quot;description&quot;: &quot;The person's last name.&quot;
    },
    &quot;age&quot;: {
      &quot;description&quot;: &quot;Age in years which must be equal to or greater than zero.&quot;,
      &quot;type&quot;: &quot;integer&quot;,
      &quot;minimum&quot;: 0
    }
  }
}
&quot;&quot;&quot;
boolean successful = WS.validateJsonAgainstSchema(response,jsonPass)</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
