"use client"
import { MyProfile } from "@/components/profile";
import { useMyProfile } from "@/hooks/useEmploye";

export default function ProfilePage() {
  const { myProfile } = useMyProfile();
  console.log("Voici votre profile: ", myProfile);
  return (
    <div className=" h-full flex-1 flex-col space-y-8 p-8 md:flex">
      <MyProfile user={myProfile} />
    </div>
  );
}
