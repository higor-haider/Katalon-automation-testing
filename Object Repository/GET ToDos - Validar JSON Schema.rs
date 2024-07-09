<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>GET ToDos - Validar JSON Schema</name>
   <tag></tag>
   <elementGuidId>9e4e1ebd-c6c0-4622-96e9-d7654f647c8d</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <smartLocatorEnabled>false</smartLocatorEnabled>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>true</autoUpdateContent>
   <connectionTimeout>0</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <katalonVersion>9.6.0</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <path></path>
   <restRequestMethod>GET</restRequestMethod>
   <restUrl>https://gorest.co.in/public/v2/todos</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>0</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <verificationScript>import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()

String jsonPass =
&quot;&quot;&quot;
{
  &quot;\$schema&quot;: &quot;http://json-schema.org/draft-04/schema#&quot;,
  &quot;type&quot;: &quot;array&quot;,
  &quot;items&quot;: [
    {
      &quot;type&quot;: &quot;object&quot;,
      &quot;properties&quot;: {
        &quot;id&quot;: {
          &quot;type&quot;: &quot;integer&quot;
        },
        &quot;user_id&quot;: {
          &quot;type&quot;: &quot;integer&quot;
        },
        &quot;title&quot;: {
          &quot;type&quot;: &quot;string&quot;
        },
        &quot;due_on&quot;: {
          &quot;type&quot;: &quot;string&quot;
        },
        &quot;status&quot;: {
          &quot;type&quot;: &quot;string&quot;
        }
      },
      &quot;required&quot;: [
        &quot;id&quot;,
        &quot;user_id&quot;,
        &quot;title&quot;,
        &quot;due_on&quot;,
        &quot;status&quot;
      ]
    }
  ]
}
&quot;&quot;&quot;

boolean successful = WS.validateJsonAgainstSchema(response,jsonPass)</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
