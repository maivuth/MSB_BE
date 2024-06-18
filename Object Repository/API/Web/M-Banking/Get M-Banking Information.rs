<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Get M-Banking Information</name>
   <tag></tag>
   <elementGuidId>11ecc88e-d0b2-403c-b1f5-e687b959260e</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <smartLocatorEnabled>false</smartLocatorEnabled>
   <useRalativeImagePath>false</useRalativeImagePath>
   <authorizationRequest>
      <authorizationInfo>
         <entry>
            <key>bearerToken</key>
            <value>eyJhbGciOiJIUzUxMiJ9.eyJzdWIiOiIxNjU3IiwicmVnaXN0cmF0aW9uX2lkIjoxNjU3LCJleHAiOjE3MjIzMzA4NTYsImlhdCI6MTcxODczMDg1NiwiYXV0aG9yaXR5IjoiTUFLRVJfUk9MRSJ9.yBnAJtFlLOVGUg7KWGv1hFEk5YmhVC6LAL4wQGlZgaVAC0CwNGE_f0waNvp3ix8nCspn20b0IFym_eNNxFMp6g</value>
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
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>4b73c6ff-05bb-499c-ac31-909a0db6f80b</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer eyJhbGciOiJIUzUxMiJ9.eyJzdWIiOiIxNjU3IiwicmVnaXN0cmF0aW9uX2lkIjoxNjU3LCJleHAiOjE3MjIzMzA4NTYsImlhdCI6MTcxODczMDg1NiwiYXV0aG9yaXR5IjoiTUFLRVJfUk9MRSJ9.yBnAJtFlLOVGUg7KWGv1hFEk5YmhVC6LAL4wQGlZgaVAC0CwNGE_f0waNvp3ix8nCspn20b0IFym_eNNxFMp6g</value>
      <webElementGuid>5a99bf1d-cbf2-471e-9a84-6ba4a8e86202</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>9.5.0</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <path></path>
   <restRequestMethod>GET</restRequestMethod>
   <restUrl>${baseUrl}/user/enterprise-profile/m-banking/${enterpriseProfileId}</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>0</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>'1657'</defaultValue>
      <description></description>
      <id>011a65e8-1c5d-42f9-8875-1129b822605b</id>
      <masked>false</masked>
      <name>enterpriseProfileId</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.baseUrl</defaultValue>
      <description></description>
      <id>56570b28-b6c1-4386-a413-4e5e216ad61e</id>
      <masked>false</masked>
      <name>baseUrl</name>
   </variables>
   <variables>
      <defaultValue>'eyJhbGciOiJIUzUxMiJ9.eyJzdWIiOiIxNjU3IiwicmVnaXN0cmF0aW9uX2lkIjoxNjU3LCJleHAiOjE3MjIzMzA4NTYsImlhdCI6MTcxODczMDg1NiwiYXV0aG9yaXR5IjoiTUFLRVJfUk9MRSJ9.yBnAJtFlLOVGUg7KWGv1hFEk5YmhVC6LAL4wQGlZgaVAC0CwNGE_f0waNvp3ix8nCspn20b0IFym_eNNxFMp6g'</defaultValue>
      <description></description>
      <id>c7ce2b58-d5a6-480a-af18-ece6634c7686</id>
      <masked>false</masked>
      <name>bearerToken</name>
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
  &quot;title&quot;: &quot;registrationAccount&quot;,
  &quot;type&quot;: &quot;object&quot;,
  &quot;properties&quot;: {
    &quot;mbankingEnterpriseProfile&quot;: {
      &quot;type&quot;: &quot;object&quot;,
      &quot;properties&quot;: {
        &quot;mbankingEnterpriseProfileList&quot;: {
            &quot;type&quot;: &quot;array&quot;,
            &quot;item&quot;: {
                &quot;additionalProperties&quot;: false,
                &quot;properties&quot; : {
                    &quot;id&quot;: {&quot;type&quot;: &quot;number&quot;},
                    &quot;fullName&quot;: {&quot;type&quot;: &quot;string&quot;},
                    &quot;idCardNumber&quot;: {&quot;type&quot;: &quot;string&quot;},
                    &quot;jobPosition&quot;: {&quot;type&quot;: [&quot;string&quot;, &quot;null&quot;]},
                    &quot;email&quot;: {&quot;type&quot;: [&quot;string&quot;, &quot;null&quot;]},
                    &quot;mobilePhoneNumber&quot;: {&quot;type&quot;: [&quot;string&quot;, &quot;null&quot;]},
                    &quot;authenticationMethod&quot;: {&quot;type&quot;: &quot;string&quot;},
                    &quot;mbankingRoles&quot;: {
                        &quot;type&quot;: &quot;array&quot;},
                        &quot;item&quot;: {
                            &quot;additionalProperties&quot;: false,
                            &quot;properties&quot; : {
                                &quot;type&quot;: &quot;string&quot;
                            }
                        }
                    },
            &quot;required&quot;: [&quot;id&quot;, &quot;fullName&quot;, &quot;idCardNumber&quot;, &quot;authenticationMethod&quot;, &quot;mbankingRoles&quot;]
            },
        &quot;required&quot;: [&quot;mbankingEnterpriseProfileList&quot;]
        },
        &quot;required&quot;: [&quot;mbankingEnterpriseProfile&quot;]
      }
    }
}
&quot;&quot;&quot;
boolean successful = WS.validateJsonAgainstSchema(response,jsonPass)
}
catch (Exception e) {
	throw &quot;API invalid&quot;
}</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
