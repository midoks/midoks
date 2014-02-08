#include <stdio.h>
/**
 *	printf
 */
#include <sys/socket.h>
#include <netinet/in.h>
#include <arpa/inet.h>
/**
 *	sockaddr_in
 *	hostent
 */

#include <string.h>
/**
 *	memset
 */
#include <sys/types.h>
#include <stdlib.h>
#include <netdb.h>
/**
 *	 gethostbyname
 */
//#include <stdarg.h>
//#include <unistd.h>
//#include <fcntl.h>

//////////////////////////////////////////
/**
 *	@func socket连接方法
 *	@param host 连接地址
 *	@param port 端口
 */
int Socket(const char *host, int port)
{
	int sock;
	unsigned long inaddr;
	struct sockaddr_in ad;
	struct hostent *hp;

	//分配内存
	memset(&ad, 0, sizeof(ad));
	ad.sin_family = AF_INET;

	//域名网络转化为二进制地址
	inaddr = inet_addr(host);
	
	if(inaddr != INADDR_NONE)
	{
		memcpy(&ad.sin_addr, &inaddr, sizeof(inaddr));
	}
	else
	{
		hp = gethostbyname(host);
		if(NULL == hp)
		{
			return -1;
		}
		//printf("inaddr_rr:%d\r\n", hp->h_addr);
		memcpy(&ad.sin_addr, hp->h_addr, hp->h_length);
	}
	ad.sin_port = htons(port);

	sock = socket(AF_INET, SOCK_STREAM, 0);
	if(sock < 0)
	{
		return sock;
	}

	if(connect(sock, (struct sockaddr *)&ad, sizeof(ad)) < 0)
	{
		//printf("host:%s port:%u", host, port);
		return -1;
	}
	//printf("host:%s:port:%u", host, port, inaddr);	
	return sock;
}
