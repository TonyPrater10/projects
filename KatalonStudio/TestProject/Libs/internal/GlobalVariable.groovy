package internal

import com.kms.katalon.core.configuration.RunConfiguration
import com.kms.katalon.core.main.TestCaseMain


/**
 * This class is generated automatically by Katalon Studio and should not be modified or deleted.
 */
public class GlobalVariable {
     
    /**
     * <p></p>
     */
    public static Object Password
     
    /**
     * <p></p>
     */
    public static Object NoSSOURL
     
    /**
     * <p></p>
     */
    public static Object DashboardURL
     
    /**
     * <p></p>
     */
    public static Object SSOURL
     

    static {
        try {
            def selectedVariables = TestCaseMain.getGlobalVariables("default")
			selectedVariables += TestCaseMain.getGlobalVariables(RunConfiguration.getExecutionProfile())
            selectedVariables += RunConfiguration.getOverridingParameters()
    
            Password = selectedVariables['Password']
            NoSSOURL = selectedVariables['NoSSOURL']
            DashboardURL = selectedVariables['DashboardURL']
            SSOURL = selectedVariables['SSOURL']
            
        } catch (Exception e) {
            TestCaseMain.logGlobalVariableError(e)
        }
    }
}
