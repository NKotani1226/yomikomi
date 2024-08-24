import { NextRequest,NextResponse} from 'next/server';

export async function GET(req:NextRequest){
    const { searchParams } = new URL(req.url);
    const url = `http://server:8080/`;
    const res = await fetch(url);
    const weather = await res.json();
    return Response.json({ weather });
}