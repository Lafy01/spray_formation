"use client"
import { Card, CardHeader, CardTitle, CardContent } from "@/components/ui/card";
import { ToastAction } from "@/components/ui/toast";
import { toast as ToastShadcn } from "@/components/ui/use-toast";
import { useAllEmployes } from "@/hooks/useAllEmploye";
import { useAllConge } from "@/hooks/useConge";
import { useMyProfile } from "@/hooks/useEmploye";
import { useAuthStore } from "@/stores/AuthStore";
import { useRouter } from "next/navigation";
import { toast } from "sonner";

export default function DashboardPage() {
  const { myProfile, error } = useMyProfile();
  const { employes } = useAllEmployes();
  const { conge } = useAllConge();

  const congeApprouver = conge.filter((conge) => conge.status_cong.includes("Approuver")).length
  const congeRefuser = conge.filter((conge) => conge.status_cong.includes("Refuser")).length
  const congeAttente = conge.filter((conge) => conge.status_cong.includes("RH")).length

  const router = useRouter();
  const logout = useAuthStore((state) => state.logout);
  const handleLogout = () => {
    toast.success("Deconnexion reussie");
    logout();
    router.push('/authentification');
    // window.location.reload();
  }

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
            {/* <DollarSign className="h-4 w-4 text-muted-foreground" /> */}
          </CardHeader>
          <CardContent>
            <div className="text-2xl font-bold">+{employes.length}</div>
            <p className="text-xs text-muted-foreground">
              Nombre des employes
            </p>
          </CardContent>
        </Card>
        <Card x-chunk="dashboard-01-chunk-1">
          <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
            <CardTitle className="text-sm font-medium">
              Conge Approuver
            </CardTitle>
            {/* <Users className="h-4 w-4 text-muted-foreground" /> */}
          </CardHeader>
          <CardContent>
            <div className="text-2xl font-bold">+{congeApprouver}</div>
            <p className="text-xs text-muted-foreground">
              Total des conge approuver
            </p>
          </CardContent>
        </Card>
        <Card x-chunk="dashboard-01-chunk-2">
          <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
            <CardTitle className="text-sm font-medium">Conge Refuser</CardTitle>
            {/* <CreditCard className="h-4 w-4 text-muted-foreground" /> */}
          </CardHeader>
          <CardContent>
            <div className="text-2xl font-bold">{congeRefuser}</div>
            <p className="text-xs text-muted-foreground">
              Total des conge refuser
            </p>
          </CardContent>
        </Card>
        <Card x-chunk="dashboard-01-chunk-3">
          <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
            <CardTitle className="text-sm font-medium">Conge en Attente</CardTitle>
            {/* <Activity className="h-4 w-4 text-muted-foreground" /> */}
          </CardHeader>
          <CardContent>
            <div className="text-2xl font-bold">{congeAttente}</div>
            <p className="text-xs text-muted-foreground">
              Total des conge en attente
            </p>
          </CardContent>
        </Card>
      </div>
    </>
  );
}
