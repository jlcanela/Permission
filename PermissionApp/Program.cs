// See https://aka.ms/new-console-template for more information
using System.Net.NetworkInformation;
using PermissionLib;
using System.IO;
using System.Text;
using System.Security.Cryptography;
using System.Xml.Serialization;
internal class Program
{

    private static void checkDllInit(string policies, string schema) {
        var success = Permission.init(ref policies, ref schema);
        if (!success) {
            Console.WriteLine("issue when initializing Permission API");
        } else {
            Console.WriteLine("PermissionLib successful init");
        }
    }
    private static void checkAllowSearchProduct(string policies, string schema) {
        var principal = "OnlineStore::User::\"unknown\"";
        var action = "OnlineStore::Action::\"searchProduct\"";
        var resource = "OnlineStore::Product::\"unknown\"";
        var context = "{ \"sessionId\": \"123456\" }";
        var entities = "[]";

        var checkAllow = Permission.check(ref policies, ref schema, ref principal, ref action, ref resource, ref context, ref entities);
        if (!checkAllow) {
            Console.WriteLine("Issue when checking Allow Permission");
        } else {
            Console.WriteLine("Check Allow succeeded");
        }

    }

    private static void checkDenySearchProduct(string policies, string schema) {
        var principal = "OnlineStore::User::\"unknown\"";
        var action = "OnlineStore::Action::\"searchProduct\"";
        var resource = "OnlineStore::Product::\"unknown\"";
        var context = "{}";
        var entities = "[]";

        var checkDeny = !Permission.check(ref policies, ref schema, ref principal, ref action, ref resource, ref context, ref entities);
        if (!checkDeny) {
            Console.WriteLine("Issue when checking Deny Permission");
        } else {
            Console.WriteLine("Check Deny succeeded");
        }

    }
    private static void Main(string[] args)
    {
        Console.WriteLine("Executing PermissionLib");

        var policies = File.ReadAllText("onlinestore.cedar"); 
        var schema = File.ReadAllText("onlinestore.cedarschema"); 
       
        checkDllInit(policies, schema);
        checkAllowSearchProduct(policies, schema);
        checkDenySearchProduct(policies, schema);

        Permission.clean();

    }
}