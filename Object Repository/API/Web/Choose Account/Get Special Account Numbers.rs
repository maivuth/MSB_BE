<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Get Special Account Numbers</name>
   <tag></tag>
   <elementGuidId>ecda125b-b28b-43db-95bc-336f4a706714</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <smartLocatorEnabled>false</smartLocatorEnabled>
   <useRalativeImagePath>false</useRalativeImagePath>
   <authorizationRequest>
      <authorizationInfo>
         <entry>
            <key>bearerToken</key>
            <value>${bearerToken}</value>
         </entry>
      </authorizationInfo>
      <authorizationType>Bearer</authorizationType>
   </authorizationRequest>
   <autoUpdateContent>true</autoUpdateContent>
   <connectionTimeout>0</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer ${bearerToken}</value>
      <webElementGuid>1073912d-497c-434b-958d-24ee9aa1a581</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>9.5.0</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <path></path>
   <restRequestMethod>GET</restRequestMethod>
   <restUrl>${baseUrl}/user/special-account-numbers?lastNumbers=${lastNumbers}</restUrl>
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
      <id>c3850074-5da9-4d4d-8c9f-aad3f023f22f</id>
      <masked>false</masked>
      <name>baseUrl</name>
   </variables>
   <variables>
      <defaultValue>'eyJhbGciOiJIUzUxMiJ9.eyJzdWIiOiIxNDgxIiwicmVnaXN0cmF0aW9uX2lkIjoxNDgxLCJleHAiOjE3MjIyMTM5MDcsImlhdCI6MTcxODYxMzkwNywiYXV0aG9yaXR5IjoiTUFLRVJfUk9MRSJ9.QfVBEVdSrk3g30yfqiXD_2KxZYE74aBAM60V2janvLX2tyTz-wlxeA2D_oTBmyKsUBB9odVg7slI1eflWIMG6w'</defaultValue>
      <description></description>
      <id>a18f2897-8a6d-4aeb-9e87-5a894e1f0ad5</id>
      <masked>false</masked>
      <name>bearerToken</name>
   </variables>
   <variables>
      <defaultValue>'468'</defaultValue>
      <description></description>
      <id>7e16e65d-4d41-4b3c-ad1a-a4bffced6817</id>
      <masked>false</masked>
      <name>lastNumbers</name>
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

try {
	WS.verifyResponseStatusCode(response, 200)
	
	assertThat(response.getStatusCode()).isEqualTo(200)
	
	String jsonPass =
	&quot;&quot;&quot;
{
  &quot;\$id&quot;: &quot;https://example.com/person.schema.json&quot;,
  &quot;\$schema&quot;: &quot;https://json-schema.org/draft/2020-12/schema&quot;,
  &quot;title&quot;: &quot;DistrictCodeList&quot;,
  &quot;type&quot;: &quot;object&quot;,
  &quot;properties&quot;: {
    &quot;specialAccountNumbers&quot;: {
      &quot;type&quot;: &quot;array&quot;,
      &quot;item&quot;: {
        &quot;type&quot;: &quot;string&quot;
      }
    }
  },
  &quot;required&quot;: [&quot;specialAccountNumbers&quot;]
}
&quot;&quot;&quot;
	boolean successful = WS.validateJsonAgainstSchema(response,jsonPass)
	}
	catch (Exception e){
		throw &quot;API failed&quot;
	}</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
