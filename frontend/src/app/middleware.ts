import { NextRequest, NextResponse } from "next/server";

export function middleware(request: NextRequest) {
    const token = request.cookies.get('token') || '';

    const url = request.nextUrl.clone();

    if (!token) {
        return NextResponse.redirect(new URL('/authentification', request.url));
    }

    return NextResponse.next();
}

export const config = {
    matcher: ['/admin/:path*', '/formateur/:path*', '/employe/:path*']
}