/**
 * 浮点型相乘
 * @param arg1
 * @param arg2
 */
function FloatMul(arg1, arg2) {
    try {
        var m = 0, s1 = arg1.toString(), s2 = arg2.toString();
        try { m += s1.split(".")[1].length } catch (e) { }
        try { m += s2.split(".")[1].length } catch (e) { }
        return Number(s1.replace(".", "")) * Number(s2.replace(".", "")) / Math.pow(10, m);
    }
    catch (e) {
        return 0;
    }
	
}

/**
 * 浮点型相加
 * @param arg1
 * @param arg2
 */
function FloatAdd(arg1, arg2) {
    var r1, r2, m;
    try { r1 = arg1.toString().split(".")[1].length } catch (e) { r1 = 0 }
    try { r2 = arg2.toString().split(".")[1].length } catch (e) { r2 = 0 }
   
    with(Math)
    {
    	n = Math.max(r1,r2);
    	m = Math.pow(10, Math.max(r1, r2));    
    	return ((arg1 * m + arg2 * m) / m).toFixed(n);
    }
}

/**
 * 浮点型相减
 * @param arg1
 * @param arg2
 */
function FloatDel(arg1, arg2) {
    var r1, r2, m;
    try { r1 = arg1.toString().split(".")[1].length } catch (e) { r1 = 0 }
    try { r2 = arg2.toString().split(".")[1].length } catch (e) { r2 = 0 }
    with(Math)
    {
        n = Math.max(r1,r2);
        m = Math.pow(10, n);
        return ((arg1  * m - arg2 * m) / m).toFixed(n);
    }
}

function FloatDiv(arg1, arg2) {
    var t1 = 0, t2 = 0, r1, r2;
    try { t1 = arg1.toString().split(".")[1].length } catch (e) { }
    try { t2 = arg2.toString().split(".")[1].length } catch (e) { }
    with (Math) {
        r1 = Number(arg1.toString().replace(".", ""))
        r2 = Number(arg2.toString().replace(".", ""))
        return (r1 / r2) * pow(10, t2 - t1);
    }
}
