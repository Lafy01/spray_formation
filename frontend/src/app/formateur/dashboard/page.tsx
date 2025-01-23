"use client"
import { Card, CardHeader, CardTitle, CardContent } from "@/components/ui/card";
import { ToastAction } from "@/components/ui/toast";
import { toast as ToastShadcn } from "@/components/ui/use-toast";
import { useAllMyEmployes } from "@/hooks/useAllEmploye";
import { useAllMyEmployeConge } from "@/hooks/useConge";
import { useMyProfile } from "@/hooks/useEmploye";
import { useAuthStore } from "@/stores/AuthStore";
import { useRouter } from "next/navigation";
import { toast } from "sonner";

export default function DashboardPage() {
  const { myProfile, error } = useMyProfile();

  const { myEmployes } = useAllMyEmployes()
  const { myEmployeConge } = useAllMyEmployeConge()

  const router = useRouter();
  const logout = useAuthStore((state) => state.logout);
  const handleLogout = () => {
    toast.success("Deconnexion reussie");
    logout();
    router.push('/authentification');
    // window.location.reload();
  }

  const congeEmployeApprouver = myEmployeConge.filter((conge) => conge.status_cong.includes("Approuver")).length
  const congeEmployeRefuser = myEmployeConge.filter((conge) => conge.status_cong.includes("Refuser")).length
  const congeEmployeAttente = myEmployeConge.filter((conge) => conge.status_cong.includes("DTP")).length

  if (error) {
    ToastShadcn({
      variant: "destructive",
      title: "Uh oh! Quelque chose ne va pas.",
      description: "Impossible de recuperer votre information.",
      action: <ToastAction onClick={handleLogout} altText="Reconnecter">Reconnecter</ToastAction>,
    })
  }

  return (
    <>
      <div className="grid gap-4 md:grid-cols-2 md:gap-8 lg:grid-cols-4">
        <Card x-chunk="dashboard-01-chunk-0">
          <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
            <CardTitle className="text-sm font-medium">
              Total Employe 
            </CardTitle>
          </CardHeader>
          <CardContent>
            <div className="text-2xl font-bold">+{myEmployes.length}</div>
<p className="text-xs text-muted-foreground">
            Total mon Employe
            </p>
          </CardContent>
        </Card>
        <Card x-chunk="dashboard-01-chunk-2">
          <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
            <CardTitle className="text-sm font-medium">Mon Solde</CardTitle>
            {/* <CreditCard className="h-4 w-4 text-muted-foreground" /> */}
          </CardHeader>
          <CardContent>
            <div className="text-2xl font-bold">+{myProfile?.solde.j_reste_sld}</div>
            <p className="text-xs text-muted-foreground">
              Mon Solde de Conge
            </p>
          </CardContent>
        </Card>
        <Card x-chunk="dashboard-01-chunk-1">
          <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
            <CardTitle className="text-sm font-medium">
              Conge En attente
            </CardTitle>
            {/* <Users className="h-4 w-4 text-muted-foreground" /> */}
          </CardHeader>
          <CardContent>
            <div className="text-2xl font-bold">+{congeEmployeAttente}</div>
            <p className="text-xs text-muted-foreground">
              total Conge employe en attente 
            </p>
          </CardContent>
        </Card>
        <Card x-chunk="dashboard-01-chunk-2">
          <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
            <CardTitle className="text-sm font-medium">Conge Refuser</CardTitle>
            {/* <CreditCard className="h-4 w-4 text-muted-foreground" /> */}
          </CardHeader>
          <CardContent>
            <div className="text-2xl font-bold">+{congeEmployeRefuser}</div>
            <p className="text-xs text-muted-foreground">
              total Conge employe refuser
            </p>
          </CardContent>
        </Card>
        <Card x-chunk="dashboard-01-chunk-3">
          <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
            <CardTitle className="text-sm font-medium">Conge Approuver</CardTitle>
            {/* <Activity className="h-4 w-4 text-muted-foreground" /> */}
          </CardHeader>
          <CardContent>
            <div className="text-2xl font-bold">+{congeEmployeApprouver}</div>
            <p className="text-xs text-muted-foreground">
            Total conge approuver
            </p>
          </CardContent>
        </Card>
      </div>
    </>
  );
}