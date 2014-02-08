/**
 * @func web stress testing
 * @Web服务压力测试工具(linux)
 * @author:midoks
 */
#include "socket.c"
#include <getopt.h>
#include <sys/param.h>
#include <signal.h>
#include <unistd.h>
#include <time.h>
//#include "func.c"

/* common info */
#define VERSION "1.0"
#define REQUEST_SIZE 2048

#ifndef MAXHOSTNAMELEN
	#define MAXHOSTNAMELEN 1000
#endif

/* ALLOW:GET, HEAD, OPTIONS, TRACE */
#define METHOD_GET 0
#define METHOD_HEAD 1
#define METHOD_OPTIONS 2
#define METHOD_TRACE 3

//FILE BUFFER
#define READ_BUFFER_COUNT 100000

//全局变量
volatile int timerexpired=0;
int clients=1;//默认并发数
int benchtime=30;//默认连接时间

int force=0;
int force_reload=0;
int proxyport=80;//默认端口
char *proxyhost=NULL;
int http10=1;/* 0 - http/0.9, 1 - http/1.0, 2 - http/1.1 */

int bytes=0;
int failed=0;
int speed=0;

int method=METHOD_GET;
char host[MAXHOSTNAMELEN];//主机信息,www.qq.ocm
char request[REQUEST_SIZE];//请求信息
int mypipe[2];
char buf[READ_BUFFER_COUNT];
int keep=0;
int keep_alive=5;//保持默认连接


//建立结构体
//getopt_lang
static const struct option long_options[]=
{
	{"clients", required_argument, NULL, 'c'},
	{"time",required_argument, NULL, 't'},
	{"version",no_argument, NULL, 'v'},
	{"keep",no_argument, NULL, 'k'},
	{NULL, 0, NULL, 0}
};

//使用方法
static void build_request(const char *url);
static int bench(void);
static void benchcore(const char* host,const int port, const char *request);
static void alarm_handler(int signal)
{
   timerexpired=1;
}

//提示语句
static void notice(void){
	fprintf(stderr,
		"webbench [option]... URL\n"
		"-c|--clients <n>        	Run <n> HTTP clients at once. Default one.\n"
		"-t|--time <sec>         	Run benchmark for <sec> seconds. Default 30.\n"
		"-v|--version            	Display program version.\n"
		"-k|--keep <sec>			keep-alive <n> sec\n"
		"-p|--proxy <server:port> 	Use proxy server for request.\n"
	);
}


int main(int argc, char *argv[])
{
	int opt = 0;
	int options_index = 0;
	char *tmp = NULL;


	//提示语句
	if(argc ==1 ){
		notice();
		return 2;
	}
	//Socket("www.com", 80);
	//printf("%d\n", id);
	while((opt = getopt_long(argc, argv, "k:p:t:c:?v", long_options, &options_index)) != EOF)
	{	
		switch(opt){
			case 0:break;
			case 'c': clients=atoi(optarg);break;
			case 't': benchtime=atoi(optarg);break;
			case '?': notice();exit(0);break;
			case 'v': printf("Wst "VERSION"\n");exit(2);break;
			case 'p': 
					/* 解析 server:port */
					tmp = strrchr(optarg, ':');
					proxyhost = optarg;
					if(tmp == NULL)
					{
						break;
					}
					if(tmp==optarg)
	     			{
		     			fprintf(stderr,"Error in option --proxy %s: Missing hostname.\n",optarg);
		     			return 2;
	     			}
	     			if(tmp==optarg+strlen(optarg)-1)
	     			{
		     			fprintf(stderr,"Error in option --proxy %s Port number is missing.\n",optarg);
		     			return 2;
	     			}
	     			*tmp='\0';
	     			proxyport=atoi(tmp+1);break;
			case 'k':keep=1;keep_alive=atoi(optarg);
					 //printf("keep:%d\nkeep_alive:%d\n", keep, keep_alive);exit(2);
					 break;
		}
	}

	//URL地址没有
	if(optind == argc){
		fprintf(stderr,	"Wst: Missing URL!\n");
		notice();
		exit(2);
	}

	fprintf(stderr, "		Wst - Web Stress Testing "VERSION" \n"
					"Copyright (c) midoks 2013 Start, Open Source Software.\n"
		);

	build_request(argv[optind]);
	printf("\nBenchmarking: "); //测试结果显示
	
	switch(method)
	{
		case METHOD_GET:
		default: printf("GET");break;
		case METHOD_OPTIONS: printf("OPTIONS");break;
		case METHOD_HEAD: printf("HEAD");break;
		case METHOD_TRACE: printf("TRACE");break;
	}
	printf(" %s\n", argv[optind]);

	//使用的HTTP协议
	switch(http10)
	{
		case 0: printf(" (using HTTP/9.0)");break;
		case 2: printf(" (using HTTP/1.1)");break;
	}

	printf("\n");

	if(clients == 1)
	{
		printf("1 client");
	}
	else
	{
		printf("%d clients", clients);
	}

	printf(", running %d sec", benchtime);

	if(force)
	{
		printf(", early socket close");
	}
	
	if(proxyhost != NULL )
	{
		printf(", via proxy server %s:%d", proxyhost, proxyport);
	}

	if(force_reload)
	{
		printf(", forcing reload");
	}
	printf(".\n");
    //printf("clients:%d\nbenchtime:%d\n", clients, benchtime);
    return bench();
}


// @func 建立请求
void build_request(const char *url)
{
#define TMPLEN 10
	char tmp[TMPLEN];
	int pos;
	char s[50];
	
	bzero(host, MAXHOSTNAMELEN);
	bzero(request, REQUEST_SIZE);

	if( force_reload && proxyhost!=NULL &&http10<1){
		http10=1;
	}
	if( method==METHOD_HEAD && http10<1 ){
		http10=1;
	}
	if( method==METHOD_OPTIONS && http10<2 ){
		http10=2;
	}
	if( method==METHOD_TRACE && http10<2 ){
		http10=2;
	}

	switch(method)
	{
		default:
		case METHOD_GET:strcpy(request, "GET");break;
		case METHOD_HEAD: strcpy(request, "HEAD");break;
		case METHOD_OPTIONS: strcpy(request, "OPTIONS");break;
		case METHOD_TRACE: strcpy(request, "TRACE");break;
	}

	strcat(request, " ");

	//URL是否标准
	if(NULL == strstr(url, "://"))
	{
		fprintf(stderr,"\n%s: is not valid URL.\n", url);
		exit(2);
	}

	//长度检查
	if(strlen(url)>1500)
	{
		fprintf(stderr,"URL is too long.\n");
		exit(2);
	}

	//主机设置
	if(proxyhost==NULL)
	{
		if(0!=strncasecmp("http://", url, 7))
		{
			fprintf(stderr,"Only HTTP protocol is directly supported, set --proxy for others.\n");
			exit(2);
		}
	}
	


	//protocol/host delimiter
	pos = strstr(url, "://")-url+3;
	
	if(strchr(url+pos, '/') == NULL)
	{
 		fprintf(stderr,"\nInvalid URL syntax - hostname don't ends with '/'.\n");
        exit(2);
		
	}

	if(proxyhost==NULL)
	{
		//获取主机端口
		if(index(url+pos, ':') != NULL &&
		   index(url+pos, ':') < index(url+pos, '/')
		   )
		{
			strncpy(host, url+pos, (strchr(url+pos, ':')-url-pos));
			bzero(tmp, TMPLEN);
			//memset(tmp, '0', sizeof(tmp));
			strncpy(tmp, 
					index(url+pos, ':')+1,//这里!!!
					strchr(url+pos, '/')-index(url+pos, ':')-1);
			//printf("pos:%d\n%d\n", index(url+pos, ':'), strchr(url+pos,'/')-index(url+pos, ':')-1);
			//tmp[0]='s';
			//printf("tmp:%s\n",tmp);
			proxyport=atoi(tmp);
			//printf("proxyport:%d\n", proxyport);
			if(proxyport==0)
			{
				proxyport=80;
			}
		}
		else
		{
			strncpy(host, url+pos, strcspn(url+pos, "/"));
		}
		//printf("proxyport:%d\n",proxyport);
		
		//连接后面的请求
		strcat(request+strlen(request), url+pos+strcspn(url+pos,"/"));
	}
	else
	{
		//printf("ProxyHost=%s\nProxyPort=%d\n", proxyhost, proxyport);
		strcat(request, url);
	}

	if(http10==1)
	{
		strcat(request, " HTTP/1.0");
	}
	else if(http10==2)
	{
		strcat(request, " HTTP/1.1");	
	}

	strcat(request, "\r\n");

	if(http10>0)
		strcat(request, "User-Agent: Wst "VERSION"\r\n");

	if(proxyhost==NULL && http10>0)
	{
		strcat(request,"Host: ");
		strcat(request,host);
		strcat(request,"\r\n");
	}

	//是否缓存
	if(force_reload && proxyhost != NULL)
		strcat(request, "Pragma: no-cache\r\n");

	//关闭
	//if(http10>0)
	//{
		if(0==keep)
		{
			strcat(request, "Connection: close\r\n");
		}
		else
		{
			strcat(request, "Connection: keep-alive\r\n");
			sprintf(s, "Keep-Alive: timeout=%d, max=100\r\n", keep_alive);
			strcat(request, s);
		}
		
	//}
	//结尾处换行
	if(http10>0) strcat(request, "\r\n");
	printf("\nrequest:\n%s\n", request);	
}

/* @func 测试 */
static int bench(void)
{
 	int i,j,k;	
  	pid_t pid=0;
  	FILE *f;

  	/* check avaibility of target server */
  	i=Socket(proxyhost==NULL?host:proxyhost,proxyport);
  	if(i<0) 
	{ 
		fprintf(stderr,"\nConnect to server failed. Aborting benchmark.\n");
        return 1;
    }
  	close(i);
  	/* create pipe */
  	if(pipe(mypipe))
  	{
		perror("pipe failed.");
		return 3;
  	}

  	/* not needed, since we have alarm() in childrens */
  	/* wait 4 next system clock tick */
  	/*
  	cas=time(NULL);
 	while(time(NULL)==cas)
        sched_yield();
  	*/

  	/* fork childs */
  	for(i=0;i<clients;i++)
  	{
		pid=fork();
		if(pid <= (pid_t) 0)
	   	{
			/* child process or error*/
	        sleep(1); /* make childs faster */
		   	break;
	   	}
  	}

  	if( pid< (pid_t) 0)
  	{
    	fprintf(stderr,"problems forking worker no. %d\n",i);
	  	perror("fork failed.");
	  	return 3;
  	}

  	if(pid == (pid_t) 0)
  	{
    	/* I am a child */
		if(proxyhost==NULL)
			benchcore(host,proxyport,request);
		else
			benchcore(proxyhost,proxyport,request);

		/* write results to pipe */
		f=fdopen(mypipe[1],"w");
		if(f==NULL)
		{
			perror("open pipe for writing failed.");
			return 3;
		}
		/* fprintf(stderr,"Child - %d %d\n",speed,failed); */
		fprintf(f,"%d %d %d\n",speed,failed,bytes);
		fclose(f);
		return 0;
	} 
	else
  	{
		f=fdopen(mypipe[0],"r");
	  	if(f==NULL) 
	  	{
			perror("open pipe for reading failed.");
			return 3;
	  	}
	  	setvbuf(f, NULL, _IONBF, 0);
	  	speed=0;
        failed=0;
        bytes=0;
	  	while(1)
	  	{
			pid=fscanf(f,"%d %d %d",&i,&j,&k);
		  	if(pid<2)
            {
                fprintf(stderr,"Some of our childrens died.\n");
            	break;
            }
		  	speed+=i;
		  	failed+=j;
		  	bytes+=k;
		  	/* fprintf(stderr,"*Knock* %d %d read=%d\n",speed,failed,pid); */
		  	if(--clients==0) break;
	  	}
	  	fclose(f);
  		printf("\nSpeed = %d pages/min, %f kb/sec.\nRequests: %d susceed, %d failed.\n",
			(int)((speed+failed)/(benchtime/60.0f)),
			(float)((bytes/(float)benchtime)/1024),
			speed,
		  	failed);
  	}
  	return i;	
}

//测试
void benchcore(const char *host, const int port, const char *req){
	int rlen;
	//char buf[1500];
	int s,i;
	struct sigaction sa;

	/* setup alarm signal handler */
	sa.sa_handler = alarm_handler;
	sa.sa_flags=0;
	if(sigaction(SIGALRM,&sa,NULL))
		exit(3);
	alarm(benchtime);

	rlen=strlen(req);
	nexttry:while(1)
	{
		if(timerexpired)
		{
		   if(failed>0)
		   {
				/* fprintf(stderr,"Correcting failed by signal\n"); */
				failed--;
		   }
		   return;
		}
		s=Socket(host,port);                          
		if(s<0) { failed++;continue;} 
		if(rlen!=write(s,req,rlen)) {failed++;close(s);continue;}
		if(http10==0) 
			if(shutdown(s,1)) { failed++;close(s);continue;}
		if(force==0) 
		{
				/* read all available data from socket */
			while(1)
			{
				if(timerexpired) break; 
			  	i=read(s, buf, READ_BUFFER_COUNT);
				//printf("buf:%s\n", buf);
				/* fprintf(stderr,"%d\n",i); */
			  	if(i<0) 
				{ 
					 failed++;
					 close(s);
					 goto nexttry;
				}
			   	else
				{
				   if(i==0){ break;}
				   else
				   {
					   bytes+=i;
				   }
				}
			}
		}
		if(close(s)) {failed++;continue;}
		speed++;
	 }
}
