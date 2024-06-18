<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Get Districts</name>
   <tag></tag>
   <elementGuidId>1da3047e-3af7-4581-a667-15cc5f294cd3</elementGuidId>
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
      <webElementGuid>976212b8-7b52-4c1a-961a-cf6c7e53ef64</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>9.5.0</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <path></path>
   <restRequestMethod>GET</restRequestMethod>
   <restUrl>${baseUrl}/user/locations/districts?provinceCode=${provinceCode}</restUrl>
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
      <id>f4e195cd-cb7a-4198-848c-6dce0e6667e9</id>
      <masked>false</masked>
      <name>baseUrl</name>
   </variables>
   <variables>
      <defaultValue>'eyJhbGciOiJIUzUxMiJ9.eyJzdWIiOiIxNDgxIiwicmVnaXN0cmF0aW9uX2lkIjoxNDgxLCJleHAiOjE3MjIyMTM5MDcsImlhdCI6MTcxODYxMzkwNywiYXV0aG9yaXR5IjoiTUFLRVJfUk9MRSJ9.QfVBEVdSrk3g30yfqiXD_2KxZYE74aBAM60V2janvLX2tyTz-wlxeA2D_oTBmyKsUBB9odVg7slI1eflWIMG6w'</defaultValue>
      <description></description>
      <id>a6f36754-76ec-40d4-870b-764b06e646de</id>
      <masked>false</masked>
      <name>bearerToken</name>
   </variables>
   <variables>
      <defaultValue>'701'</defaultValue>
      <description></description>
      <id>baaade6f-747d-42b2-a843-1c6ee391ff07</id>
      <masked>false</masked>
      <name>provinceCode</name>
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
    &quot;provinceCodeList&quot;: {&quot;type&quot;: &quot;null&quot;},
    &quot;districtCodeList&quot;: {
      &quot;type&quot;: &quot;array&quot;,
      &quot;item&quot;: {
        &quot;additionalProperties&quot;: false,
        &quot;properties&quot; : {
            &quot;code&quot;: {&quot;type&quot;: &quot;string&quot;},
            &quot;description&quot;: {&quot;type&quot;: &quot;string&quot;}
        },
        &quot;required&quot;: [&quot;code&quot;, &quot;description&quot;]
      }
    },
    &quot;branchCodeList&quot;: {&quot;type&quot;: &quot;null&quot;}
  },
  &quot;required&quot;: [&quot;districtCodeList&quot;]
}
&quot;&quot;&quot;
boolean successful = WS.validateJsonAgainstSchema(response,jsonPass)
}
catch (Exception e){
	throw &quot;API failed&quot;
}</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
